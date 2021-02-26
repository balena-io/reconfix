var inherit = require( 'bloodline' )
var MBR = require( '../mbr' )

/**
 * AST (AST Research / NEC MS-DOS and SpeedStor)
 * @class
 * @extends {MBR}
 * @memberOf MBR
 * @param {Buffer} [buffer]
 * @param {Number} [start]
 * @param {Number} [end]
 * @returns {AST}
 */
function AST( buffer, start, end ) {

  if( !(this instanceof AST) )
    return new AST( buffer, start, end )

  MBR.call( this, buffer, start, end )

}

/**
 * Partition table offset
 * @const {Number}
 */
AST.TABLE_OFFSET = 0x17E

/**
 * Partition table entry count
 * @const {Number}
 */
AST.PARTITION_ENTRIES = 8

/**
 * AST prototype
 * @type {Object}
 * @ignore
 */
AST.prototype = {

  constructor: AST,

  get buffer() {

    var buffer = MBR.createBuffer()
    var offset = this.tableOffset

    for( var i = 0; i < this.code.length; i++ ) {
      this.code[i].data.copy( buffer, this.code[i].offset )
    }

    // Write out extended partition records
    this.partitions[4].buffer.copy( buffer, offset )
    this.partitions[5].buffer.copy( buffer, offset += 0x10 )
    this.partitions[6].buffer.copy( buffer, offset += 0x10 )
    this.partitions[7].buffer.copy( buffer, offset += 0x10 )

    // Write out primary partition records
    this.partitions[0].buffer.copy( buffer, offset += 0x10 )
    this.partitions[1].buffer.copy( buffer, offset += 0x10 )
    this.partitions[2].buffer.copy( buffer, offset += 0x10 )
    this.partitions[3].buffer.copy( buffer, offset += 0x10 )

    return buffer

  },

  set buffer( value ) {

    if( !Buffer.isBuffer( value ) )
      throw new TypeError( 'Value must be a Buffer' )

    this.code = [
      new MBR.Code( value, 0, 0x17E )
    ]

    var i, count = 4
    var offset = 0x1BE

    // Read in primary partitions first
    for( i = 0; i < count; i++ ) {
      this.partitions[i].buffer =
        value.slice( offset, offset += 0x10 )
    }

    count += count
    offset = this.tableOffset

    // Then extended partition entries,
    // to keep the array order sane
    for( i = 4; i < count; i++ ) {
      this.partitions[i].buffer =
        value.slice( offset, offset += 0x10 )
    }

  },

}

inherit( AST, MBR )
// Exports
module.exports = AST
