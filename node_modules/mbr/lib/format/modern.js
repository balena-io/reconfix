var inherit = require( 'bloodline' )
var MBR = require( '../mbr' )

/**
 * MODERN (modern standard)
 * @class
 * @extends {MBR}
 * @memberOf MBR
 * @param {Buffer} [buffer]
 * @param {Number} [start]
 * @param {Number} [end]
 */
function MODERN( buffer, start, end ) {

  if( !(this instanceof MODERN) )
    return new MODERN( buffer, start, end )

  // In order to ensure the integrity of the MBR boot loader code,
  // it is important that the bytes at +0DAh to +0DFh are never changed,
  // unless either all six bytes represent a value of 0 or the whole MBR
  // bootstrap loader code (except for the (extended) partition table) is
  // replaced at the same time as well. This includes resetting these values
  // to 00h 00h 00h 00h 00h 00h unless the code stored in the MBR is known.
  // NOTE: Windows adheres to this rule.

  /** @type {Number} Original physical drive (80h-FFh) */
  this.physicalDrive = 0x80
  /** @type {Object} Disk timestamp (optional) */
  this.timestamp = {
    seconds: 0,
    minutes: 0,
    hours: 0,
  }

  /** @type {Number} Disk signature (32bit, optional) */
  this.signature = null
  /** @type {Boolean} Copy Protection */
  this.copyProtected = false

  MBR.call( this, buffer, start, end )

}

/**
 * Partition table offset
 * @const {Number}
 */
MODERN.TABLE_OFFSET = 0x1BE

/**
 * Partition table entry count
 * @const {Number}
 */
MODERN.PARTITION_ENTRIES = 4

/**
 * MODERN prototype
 * @type {Object}
 * @ignore
 */
MODERN.prototype = {

  constructor: MODERN,

  get buffer() {

    var buffer = MBR.createBuffer()

    for( var i = 0; i < this.code.length; i++ ) {
      this.code[i].data.copy( buffer, this.code[i].offset )
    }

    buffer[ 0x0DC ] = this.physicalDrive
    buffer[ 0x0DD ] = this.timestamp.seconds
    buffer[ 0x0DE ] = this.timestamp.minutes
    buffer[ 0x0DF ] = this.timestamp.hours

    // If second code block includes the disk signature area,
    // don't write it out (TODO: find documentation on copy protection)
    if( this.code[1] && this.code[1].data.length === 216 ) {

      buffer.writeUInt32LE( this.signature, 0x1B8 )

      this.copyProtected ?
        buffer.writeUInt16LE( 0x5A5A, 0x1BC ) :
        buffer.writeUInt16LE( 0x0000, 0x1BC )

    }

    var offset = this.tableOffset

    this.partitions[0].buffer.copy( buffer, offset )
    this.partitions[1].buffer.copy( buffer, offset += 0x10 )
    this.partitions[2].buffer.copy( buffer, offset += 0x10 )
    this.partitions[3].buffer.copy( buffer, offset += 0x10 )

    return buffer

  },

  set buffer( value ) {

    if( !Buffer.isBuffer( value ) )
      throw new TypeError( 'Value must be a Buffer' )

    this.physicalDrive = value.readUInt8( 0x0DC )

    this.timestamp.seconds = value[ 0x0DD ]
    this.timestamp.minutes = value[ 0x0DE ]
    this.timestamp.hours = value[ 0x0DF ]

    var marker = value.readUInt16LE( 0x1BC )

    // If unint16 @ 0x1BC == 0x5A5A,
    // then it's copy protected (?)
    this.copyProtected = marker === 0x5A5A

    // TODO: Figure out why I'm checking for 0x1BC to be zero
    // before attempting to read a signature
    // (it most likely has to do with the EOC marker)
    this.signature = marker === 0 || this.copyProtected ?
      value.readUInt32LE( 0x1B8 ) : null

    var endOfCode = this.signature == null ?
      0x1BE : 0x1B8

    this.code = [
      new MBR.Code( value, 0, 0x0DA ),
      new MBR.Code( value, 0x0E0, endOfCode ),
    ]

    var i, count = this.partitionEntries
    var offset = this.tableOffset

    for( i = 0; i < count; i++ ) {
      this.partitions[i].buffer =
        value.slice( offset, offset += 0x10 )
    }

  },

}

inherit( MODERN, MBR )
// Exports
module.exports = MODERN
