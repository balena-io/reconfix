var inherit = require( 'bloodline' )
var MBR = require( '../mbr' )

/**
 * AAP (Advanced Active Partitions)
 * @class
 * @extends {MBR}
 * @memberOf MBR
 * @param {Buffer} [buffer]
 * @param {Number} [start]
 * @param {Number} [end]
 */
function AAP( buffer, start, end ) {

  if( !(this instanceof AAP) )
    return new AAP( buffer, start, end )

  /** @type {MBR.AAP.Record} AAP Record */
  this.aap = new AAP.Record()

  MBR.call( this, buffer, start, end )

}

/**
 * Partition table offset
 * @const {Number}
 */
AAP.TABLE_OFFSET = 0x1BE

/**
 * Partition table entry count
 * @const {Number}
 */
AAP.PARTITION_ENTRIES = 4

AAP.Record = require( './aap-record' )

/**
 * AAP prototype
 * @type {Object}
 * @ignore
 */
AAP.prototype = {

  constructor: AAP,

  get buffer() {

    var buffer = MBR.createBuffer()
    var offset = this.tableOffset

    for( var i = 0; i < this.code.length; i++ ) {
      this.code[i].data.copy( buffer, this.code[i].offset )
    }

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
      new MBR.Code( value, 0, 0x1AB )
    ]

    this.aap.buffer = value.slice( 0x1AE, 0x1BA )

    var i, count = this.partitionEntries
    var offset = this.tableOffset

    for( i = 0; i < count; i++ ) {
      this.partitions[i].buffer =
        value.slice( offset, offset += 0x10 )
    }

  },

}

inherit( AAP, MBR )
// Exports
module.exports = AAP
