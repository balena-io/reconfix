# Reconfix Architecture

Reconfix has been designed from the ground up to solve one very important class of problems: synchronizing heterogenous data repositories. A motivating example is synchronizing (parts of) two configuration files in different formats that are used by two different programs, and doing so in a generic and safe way.

Reconfix solves this through a reactive-style architecture, where changes are pushed through generic bidirectional transformation steps until something errors out or those changes reach all other data repositories and the synchronization is successful.

Reconfix is written in Rust and is composed of two major crates:

- `reconfix`: the core reconfix library.
- `reconfix-cli`: executable providing a command-line interface for reconfix.

And the major modules are:

- `reconfix::external_data`: support for external data repositories.
- `reconfix::lens`: support for lenses (bidirectional transforms) defined in CUE.
- `reconfix::orchestrator`: support for building and running transformation graphs.
- `reconfix::transform`: support for (unidirectional) transforms defined in CUE.

Code documentation is available through `rustdoc`.

## Lenses

In reconfix, bidirectional transforms are defined as *lenses* and written in CUE. A lens is a pair of pure functions where one is the inverse of the other. These functions are represented as a pair of mutually recursive fields `X` and `Y`. For example:

```cue
import "strconv"

X: strconv.Atoi(Y)
Y: strconv.FormatInt(X, 10)
```

Is a lens that parses a string containing a base 10 number one way, and stringifies an integer in base 10 the other way.

### Bijection

CUE lenses are *bijective* by construction. That is, if a transformation succeeds one way, it will succeed the other way. But we can define lenses that are not bijective in the usual mathematical sense, relying on CUE to check whether that lens is can invert a specific value. For example:

```cue
X: Y * 2
Y: X div 2
```

where `div` is the integral division operator. CUE will only allow this lens to be used with values that can be inverted:

- `X: 10` is ok:

```
$ cue export example.cue
{
    "X": 10,
    "Y": 5
}
```

- `X: 9` is not ok:

```
$ cue export example.cue
X: conflicting values 8 and 9:
    ./example.cue:1:4
    ./example.cue:4:4
```

### Conflict Detection

CUE implements a paradigm more similar to logic than either procedural or functional. That is, if we have a CUE lens:

```cue
X: Y * 2
Y: X / 2
```

and we set `X: 10.0` and `Y: 5`, evaluation will fail. CUE looks for the intersection of *all* evaluations instead of just one. In this case, `Y` has two definitions:

- `Y: 5`
- `Y: X / 2`

With `X: 10.0`, `X / 2` evaluates to `5.0` which is of a different type than `5`. Thus the number of possible values for `Y` is zero and CUE errors out. That way by leveraging CUE we can guarantee that external data and internal transformation are all logically consistent.

### Termination

CUE does allow cycles but does not recurse. That is, every value within CUE must be computable without recursion or looping. If all builtins terminate, CUE lenses also terminate.

### State and Side Effects

Lenses are expected to be purely functional entities free of side-effects. In CUE, stateful functions are exposed through the `tool` package and its subpackages. While it is not forbidden to use those packages, the implementation must be careful to use them in a way that abides by this assumption. Any violation of this rule may cause silent desynchronization.

**WIP: the following is still under consideration**

For some operations, it is useful to keep state between runs. For example, a projections lens:

```cue
X: Y.message
XSAVE: {
    for k, v in Y {
        if k != "message" {
            "\(k)": v
        }
    }
}

Y: XSAVE & {
    message: X
}
```

When calculating `Y` from `X` we need not only the value for the projected field, but also all non-projected fields. In this case, non-projected fields are stored in the `XSAVE` field, and `YSAVE` is also valid if needed.

While it is certainly possible to implement this functionality using stateful operations, or even `ExternalData` nodes (see the "External Data" section), `XSAVE`/`YSAVE` are both simpler and better integrated into the orchestrator's transactional nature. For more information on how `XSAVE` and `YSAVE` work, see the "Orchestrator/Lens' State" subsection.

## Transforms

**TODO: may be a feature for v2**

## External Data

Reconfix synchronizes "external data repositories". An external data repository is any dataset that can be represented in a single JSON document. Acessing, parsing, and modifying these datasets is left to arbitrary objects implementing the `ExternalData` trait. The only thing reconfix is concerned with are receiving new values from those external data repositories, and pushing new values to other nodes as a result of a successful synchronization.

In line with the orchestrator's push-based design, `ExternalData` objects are responsible for asynchronously pushing new values as they happen. Implementors must be prepared to deal with a possible rejection appropriately. Possible responses to an error include reverting the new value and retrying with an updated value.

## Orchestration

Lenses, transforms, and `ExternalData` objects are composed into a *transformation graph*. Transformation graphs are directed, possibly cyclic graphs but the direction of edges only define composition between nodes and not any kind of order. Transformation graphs have no defined order of evaluation, there is no guarantee than any lenses or transforms will be evaluated at all, and lenses and transforms may be evaluated any number of times. Lenses and transforms are assumed to be purely functional with no side-effects, so this is not a problem.

Given a transformation graph containing at least two `ExternalData` nodes, the orchestrator can enter an event loop. The graph is immutable while the event loop is running. The event loop has a single purpose: accept new values from external data nodes and propagate those changes through the graph into all other external data nodes.

The orchestrator is transactional: a change is only synchronized and persisted if no lenses nor transforms fails, and all external data nodes accept the transformed change.

### Event Loop

At start the orchestrator gives a `Synchronizer` object to each external data node in the transformation graph. Each `ExternalData` object is expected is setup its own triggers to forward changes to the orchestrator through that `Synchronizer` object. This may involve, for example, setting up `inotify` hooks or polling tasks.

The orchestrator then awaits for an initial value from each of those objects. The orchestrator always keeps a full (and sometimes more than one) copy of the data that each external data node exposes to reconfix.

#### Delta Propagation

**TODO: may be a feature for v2**

It is probably worth investigating delta propagation instead of keeping everything in memory.

### Journaling

**WIP: the following is still under consideration**

While the orchestrator is transactional, that only applies to the internal transformations. Even if all transformations succeeds, a `commit` called on an `ExternalData` object may still fail. If other `commit`s have already succeeded, external data nodes will become desynchronized. To avoid that, the orchestrator will rollback and ultimately fail the transaction.

External data nodes that need to be rolled back are stored in an in-memory journal.

Failures inside a rollback are fatal and will leave some external data nodes in a desynchronized state.

#### Persistent Journal

**TODO: may be a feature for v2**

It may be worth persisting the journal so reconfix can recover from power failures.

### Conflict Resolution

**TODO: at least some form of authoritative/non-authoritative resolution**

### Lens' State

**WIP: the following is still under consideration**

Lenses may define `XSAVE` and `YSAVE` fields. These two fields are independent and optional, and when present they are saved and restored depending on which field is being evaluated:

- When evaluating `X` from a concrete `Y`, `YSAVE` is restored and `XSAVE` is saved.
- When evaluating `Y` from a concrete `X`, `XSAVE` is restored and `YSAVE` is saved.

Take the projection lens as an example:

```cue
X: Y.message
XSAVE: {
    for k, v in Y {
        if k != "message" {
            "\(k)": v
        }
    }
}

Y: XSAVE & {
    message: X
}
```

If we are evluating `X`, `Y` will be set to some concrete value. `XSAVE` will also be evaluated, and CUE will also check that the inverse transformation is valid. If the transaction succeeds, `XSAVE` will be saved for future runs.

On the other hand if we are evaluating `Y`, `X` will be set to some concrete value. `XSAVE` will also be set to the concrete value that was last saved.

### Automatic Assembly

**TODO: planned feature for v2**
