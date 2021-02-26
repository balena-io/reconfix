var inherit = require( 'bloodline' )
var MBR = require( '../mbr' )

/**
 * NEWLDR
 * @class
 * @extends {MBR}
 * @memberOf MBR
 * @param {Buffer} [buffer]
 * @param {Number} [start]
 * @param {Number} [end]
 */
function NEWLDR( buffer, start, end ) {

  if( !(this instanceof NEWLDR) )
    return new NEWLDR( buffer, start, end )

  /** @type {MBR.NEWLDR.Record} NEWLDR record */
  this.newldr = new NEWLDR.Record()
  /** @type {MBR.AAP.Record} AAP partition entry #0 with special semantics */
  this.aap = new MBR.AAP.Record()

  MBR.call( this, buffer, start, end )

}

/**
 * Partition table offset
 * @const {Number}
 */
NEWLDR.TABLE_OFFSET = 0x1BE

/**
 * Partition table entry count
 * @const {Number}
 */
NEWLDR.PARTITION_ENTRIES = 4

NEWLDR.Record = require( './newldr-record' )

/**
 * NEWLDR prototype
 * @type {Object}
 * @ignore
 */
NEWLDR.prototype = {

  constructor: NEWLDR,

  get buffer() {

    var buffer = MBR.createBuffer()
    var offset = this.tableOffset

    for( var i = 0; i < this.code.length; i++ ) {
      this.code[i].data.copy( buffer, this.code[i].offset )
    }

    this.newldr.buffer.copy( buffer )
    this.aap.buffer.copy( buffer, 0x1AE )

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
      new MBR.Code( value, value[1], 0x1AC )
    ]

    this.newldr.buffer = value.slice( 0, value[1] )
    this.aap.buffer = value.slice( 0x1AE, 0x1BE )

    var i, count = this.partitionEntries
    var offset = this.tableOffset

    for( i = 0; i < count; i++ ) {
      this.partitions[i].buffer =
        value.slice( offset, offset += 0x10 )
    }

  },

}

inherit( NEWLDR, MBR )
// Exports
module.exports = NEWLDR
