var inherit = require( 'bloodline' )
var MBR = require( '../mbr' )

/**
 * DM (Disk Manager)
 * @class
 * @extends {MBR}
 * @memberOf MBR
 * @param {Buffer} [buffer]
 * @param {Number} [start]
 * @param {Number} [end]
 * @returns {DM}
 */
function DM( buffer, start, end ) {

  if( !(this instanceof DM) )
    return new DM( buffer, start, end )

  MBR.call( this, buffer, start, end )

}

/**
 * Partition table offset
 * @const {Number}
 */
DM.TABLE_OFFSET = 0x0FE

/**
 * Partition table entry count
 * @const {Number}
 */
DM.PARTITION_ENTRIES = 16

/**
 * DM prototype
 * @type {Object}
 * @ignore
 */
DM.prototype = {

  constructor: DM,

  get buffer() {

    var buffer = MBR.createBuffer()
    var offset = this.tableOffset

    for( var i = 0; i < this.code.length; i++ ) {
      this.code[i].data.copy( buffer, this.code[i].offset )
    }

    // Add DM extended partition entries
    for( var i = 4; i < DM.PARTITION_ENTRIES; i++ ) {
      this.partitions[i].buffer.copy( buffer, offset )
      offset += 0x10
    }

    // Append primary partition entries
    for( var i = 0; i < 4; i++ ) {
      this.partitions[i].buffer.copy( buffer, offset )
      offset += 0x10
    }

    return buffer

  },

  set buffer( value ) {

    if( !Buffer.isBuffer( value ) )
      throw new TypeError( 'Value must be a Buffer' )

    this.code = [
      new MBR.Code( value, 0, 0x0FE )
    ]

    var i, count = 4
    var offset = 0x1BE

    // Read in primary partitions first
    for( i = 0; i < count; i++ ) {
      this.partitions[i].buffer =
        value.slice( offset, offset += 0x10 )
    }

    count *= 4
    offset = this.tableOffset

    // Then extended partition entries,
    // to keep the array order sane
    for( i = 4; i < count; i++ ) {
      this.partitions[i].buffer =
        value.slice( offset, offset += 0x10 )
    }

  },

}

inherit( DM, MBR )
// Exports
module.exports = DM
