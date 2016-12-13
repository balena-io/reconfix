reconfix
========

> (Re)Configuration toolkit

Reconfix is a tool to obtain a high-level configuration state out of
information scattered around different files, perform modifications to it, and
write it back.

Installation
------------

Install `reconfix` by running:

```sh
$ npm install --save reconfix
```

Documentation
-------------

Reconfix can be hard to grok. We'll discuss a concrete example in depth in
order to ease the steep learning curve.

The Raspberry Pi reads system configuration parameters from a file called
[`/boot/config.txt`][rpi-config-txt], which lives in the first partition of the
image. This file contains a set of key-value pairs following the [ini] file
format. For example:

```txt
gpu_mem=192
hdmi_drive=2
hdmi_mode=82
```

Let's pretend we wrote a YouTube video player application for our Raspberry Pi,
which also reads certain application configuration from a file called
`/etc/pitube.json`. Here's an example of our ficticious file:

```json
{
  "user": "johndoe",
  "apiKey": "xxxx-yyyy-zzzz",
  "quality": 720,
  "fullscreen": true,
  "annotations": false
}
```

Finally, let's say we've configured our Raspberry Pi to read network
configuration from a [NetworkManager][networkmanager] file called
`/etc/network` that looks something like this:

```
[home]
Type = wifi
Name = mynetwork
Passphrase = secret
Nameservers = 8.8.8.8,8.8.4.4
```

We want to build a tool that allows us to read configuration from all the files
we care about, modify it, and write it back, but the problem is that the
settings live in different files, which follow different formats. We can of
course write custom code to handle this small example, but Reconfix allows us
to tackle the problem in a declarative way.

### Phases

In order to provide bi-directional transformation of data, Reconfix makes use
of different transformation phases.

#### Dry JSON

This phase consists of a high-level configuration state taken from all the
files we care about. Based on the example we outlined at the beginning of this
section, we could design our dry JSON to look something like this:

```json
{
  "gpuMemory": <number>,
  "hdmi": {
    "drive": <number>,
    "mode": <number>
  },
  "youtube": {
    "annotations": <boolean>,
    "quality": <number>,
    "auth": {
      "user": <string>,
      "apiKey": <string>
    }
  },
  "pitube": {
    "fullscreen": <boolean>,
  },
  "wifi": {
    "ssid": <string>,
    "key": <string>
  }
}
```

A specific instance of this phase's JSON that matches the data we invented in
our example looks like this:

```json
{
  "gpuMemory": 192,
  "hdmi": {
    "drive": 2,
    "mode": 82
  },
  "youtube": {
    "annotations": false,
    "quality": 720,
    "auth": {
      "user": "johndoe",
      "apiKey": "xxxx-yyyy-zzzz"
    }
  },
  "pitube": {
    "fullscreen": true,
  },
  "wifi": {
    "ssid": "mynetwork",
    "key": "secret"
  }
}
```

#### Wet JSON

This phase consists of a transformation to the dry JSON phase so it more
closely matches the file-system structure. For example:

```json
{
  "boot_config_txt": {
    "gpu_mem": 192,
    "hdmi_drive": 2,
    "hdmi_mode": 82
  },
  "etc_pitube_json": {
    "user": "johndoe",
    "apiKey": "xxxx-yyyy-zzzz",
    "quality": 720,
    "fullscreen": true,
    "annotations": false
  },
  "etc_network": {
    "home": {
      "Name": "mynetwork",
      "Passphrase": "secret"
    }
  }
}
```

#### Files JSON

Finally, the files JSON stage converts the wet JSON into precise file locations
and data that can be written by Reconfix to the file-system. For example:

```json
{
  "boot_config_txt": {
    "location": {
      "path": "config.txt",
      "partition": {
        "primary": 1
      }
    },
    "data": "gpu_mem=192\nhdmi_drive=2\nhdmi_mode=82\n"
  },
  "etc_pitube_json": {
    "location": {
      "path": "/etc/pitube.json",
      "partition": {
        "primary": 2
      }
    },
    "data": "{\"user\":\"johndoe\",\"apiKey\":\"xxxx-yyyy-zzzz\",\"quality\":720,\"fullscreen\":true,\"annotations\":false}"
  },
  "etc_network": {
    "location": {
      "path": "/etc/network",
      "partition": {
        "primary": 2
      }
    },
    "data": "[home]\nName = mynetwork\nPassphrase = secret\n"
  }
}
```

### Schemas

Now that we understand the different transformation phases that Reconfix
implements, we can talk about schemas. Schemas are user-defined declarative
JSON files that help Reconfix to bi-directionally move between the phases
described above.

#### Dry JSON <-> Wet JSON

This schema allows Reconfix to go from the dry JSON to the wet JSON, and vice
versa. This is a valid schema for our YouTube player example:

```json
[
  {
    "template": {
      "gpu_mem": "{{gpuMemory}}",
      "hdmi_drive": "{{hdmi.drive}}",
      "hdmi_mode": "{{hdmi.mode}}"
    },
    "domain": [
      [ "boot_config_txt", "gpu_mem" ],
      [ "boot_config_txt", "hdmi_drive" ],
      [ "boot_config_txt", "hdmi_mode" ]
    ]
  },
  {
    "template": {
      "user": "{{youtube.auth.user}}",
      "apiKey": "{{youtube.auth.apiKey}}",
      "quality": "{{youtube.quality}}",
      "fullscreen": "{{pitube.fullscreen}}",
      "annotations": "{{youtube.annotations}}"
    },
    "domain": [
      [ "etc_pitube_json", "user" ],
      [ "etc_pitube_json", "apiKey" ],
      [ "etc_pitube_json", "quality" ],
      [ "etc_pitube_json", "fullscreen" ],
      [ "etc_pitube_json", "annotations" ]
    ]
  },
  {
    "template": {
      "home": {
        "Name": "{{wifi.ssid}}",
        "Passphrase": "{{wifi.key}}"
      }
    },
    "domain": [
      [ "etc_network", "home", "Name" ],
      [ "etc_network", "home", "Passphrase" ]
    ]
  }
]
```

Notice how this schema defines a relationship between the structure of the dry
JSON, and the structure of the wet JSON.

### Wet JSON <-> Files JSON

A schema to go from wet JSON to files JSON, and vice versa, would look
something like this:

```json
{
  "boot_config_txt": {
    "type": "ini",
    "location": {
      "path": "config.txt",
      "partition": {
        "primary": 1
      }
    }
  },
  "etc_pitube_json": {
    "type": "json",
    "location": {
      "path": "/etc/pitube.json",
      "partition": {
        "primary": 2
      }
    }
  },
  "etc_network": {
    "type": "ini",
    "location": {
      "path": "/etc/network",
      "partition": {
        "primary": 2
      }
    }
  }
}
```

This schema declares each file by a unique name (which is up to the user to
define), and specifies its type, and its location.

Now that everything is configured, we can make use of Reconfix functions to
read/write configuration:

### `Promise reconfix.readConfiguration(Object schema, String path)`

```js
const reconfix = require('reconfix');

reconfix.readConfiguration({
  mapper: { <dry-wet-schema> },
  files: { <wet-files-schema> }
}, '/dev/sda').then((configuration) => {

  // This is the Dry JSON
  console.log(configuration);

});
```

### `Promise reconfix.writeConfiguration(Object schema, Object configuration, String path)`

```js
const reconfix = require('reconfix');

reconfix.writeConfiguration({
  mapper: { <dry-wet-schema> },
  files: { <wet-files-schema> }
}, {
  <dry json>
}, '/dev/sda').then(() => {
  console.log('Done!');
});
```

Notice that your configuration changes will be merged to what is currently on
the device.

Support
-------

If you're having any problem, please [raise an issue][newissue] on GitHub and
the resin.io team will be happy to help.

Tests
-----

Run the test suite by doing:

```sh
$ npm test
```

License
-------

Reconfix is free software, and may be redistributed under the terms specified
in the [license][license].

[license]: https://github.com/resin-io/reconfix/blob/master/LICENSE
[newissue]: https://github.com/resin-io/reconfix/issues/new
[rpi-config-txt]: https://www.raspberrypi.org/documentation/configuration/config-txt.md
[ini]: https://en.wikipedia.org/wiki/INI_file
[networkmanager]: https://wiki.gnome.org/Projects/NetworkManager
