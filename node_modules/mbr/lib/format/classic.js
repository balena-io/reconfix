var inherit = require( 'bloodline' )
var MBR = require( '../mbr' )

/**
 * CLASSIC (classical generic)
 * @class
 * @extends {MBR}
 * @memberOf MBR
 * @param {Buffer} [buffer]
 * @param {Number} [start]
 * @param {Number} [end]
 * @returns {CLASSIC}
 */
function CLASSIC( buffer, start, end ) {

  if( !(this instanceof CLASSIC) )
    return new CLASSIC( buffer, start, end )

  MBR.call( this, buffer, start, end )

}

/**
 * Partition table offset
 * @const {Number}
 */
CLASSIC.TABLE_OFFSET = 0x1BE

/**
 * Partition table entry count
 * @const {Number}
 */
CLASSIC.PARTITION_ENTRIES = 4

/**
 * CLASSIC prototype
 * @type {Object}
 * @ignore
 */
CLASSIC.prototype = {

  constructor: CLASSIC,

  get buffer() {

    var buffer = MBR.createBuffer()
    var offset = this.tableOffset

    for( var i = 0; i < this.code.length; i++ ) {
      this.code[i].data.copy( buffer, this.code[i].offset )
    }

    this.partitions[0].buffer.copy( buffer, offset )
    this.partitions[1].buffer.copy( buffer, offset += 0x10 )
    this.partitions[2].buffer.copy( buffer, offset += 0x10 )
    this.partitions[3].buffer.copy( buffer, offset += 0x10 )

    return buffer

  },

  set buffer( value ) {

    if( !Buffer.isBuffer( value ) )
      throw new TypeError( 'Value must be a Buffer' )

    this.code = [
      new MBR.Code( value, 0, 0x1BE )
    ]

    var i, count = this.partitionEntries
    var offset = this.tableOffset

    for( i = 0; i < count; i++ ) {
      this.partitions[i].buffer =
        value.slice( offset, offset += 0x10 )
    }

  },

}

inherit( CLASSIC, MBR )
// Exports
module.exports = CLASSIC
