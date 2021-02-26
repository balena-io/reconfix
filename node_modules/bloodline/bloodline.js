/**
 * Retrieves an object's description
 * @param  {Object} object
 * @return {Object} description
 */
function describe( object ) {
  return Object.getOwnPropertyNames( object )
    .reduce( function( desc, key ) {
      desc[ key ] = Object.getOwnPropertyDescriptor( object, key )
      return desc
    }, Object.create( null ))
}

/**
 * Inherits from sctor to ctor
 * @param  {Function} ctor
 * @param  {Function} sctor
 * @return {Object} 
 */
function inherit( ctor, sctor ) {
  ctor.prototype = Object.create(
    sctor.prototype,
    describe( ctor.prototype )
  )
}

module.exports = inherit
