var CHS = require( 'chs' )

/**
 * AAP Record
 * @class
 * @memberOf MBR.AAP
 * @param {Buffer} [buffer]
 * @param {Number} [start]
 * @param {Number} [end]
 */
function Record( buffer, start, end ) {

  if( !(this instanceof Record) )
    return new Record( buffer, start, end )

  /** @type {Number} AAP physical drive (80h-FEh; 00h: not used; 01h-7Fh, FFh: reserved) */
  this.physicalDrive = 0x00
  /** @type {CHS} CHS (start) address of AAP partition/image file or VBR/EBR */
  this.firstCHS = new CHS()
  /** @type {Number} AAP partition type (00h if not used) (optional) */
  this.partitionType = 0x00
  /** @type {CHS} CHS end address in AAP (optional, 000000h if not used) */
  this.lastCHS = new CHS()
  /** @type {Number} Start LBA of AAP image file or VBR/EBR */
  this.firstLBA = 0x00000000
  /** @type {Number} Reserved for sectors in AAP (optional; 00000000h if not used) */
  this.sectors = 0x00000000

  if( Buffer.isBuffer( buffer ) ) {
    this.buffer = buffer.slice( start, end )
  }

}

/**
 * AAP Record Prototype
 * @type {Object}
 * @ignore
 */
Record.prototype = {

  constructor: Record,

  /**
   * Buffer
   * @property {Buffer} buffer
   */
  get buffer() {

    var buffer = new Buffer( 0x10 )

    buffer.writeUInt8( this.physicalDrive, 0x00 )
    buffer.writeUInt8( this.partitionType, 0x04 )
    buffer.writeUInt32LE( this.firstLBA, 8 )
    buffer.writeUInt32LE( this.sectors, 12 )

    this.firstCHS.buffer.copy( buffer, 1 )
    this.lastCHS.buffer.copy( buffer, 5 )

    return buffer

  },

  set buffer( value ) {

    if( !Buffer.isBuffer( value ) )
      throw new TypeError( 'Value must be a Buffer' )

    this.physicalDrive   = value.readUInt8( 0x00 )
    this.firstCHS.buffer = value.slice( 1, 4 )
    this.partitionType   = value.readUInt8( 0x04 )
    this.lastCHS.buffer  = value.slice( 5, 8 )
    this.firstLBA        = value.readUInt32LE( 8 )
    this.sectors         = value.readUInt32LE( 12 )

    return this

  },

}

// Exports
module.exports = Record
