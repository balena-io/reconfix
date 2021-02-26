var CHS = require( 'chs' )

/**
 * NEWLDR Record
 * @class
 * @memberOf MBR.NEWLDR
 * @param {Buffer} [buffer]
 * @param {Number} [start]
 * @param {Number} [end]
 */
function Record( buffer, start, end ) {

  if( !(this instanceof Record) )
    return new Record( buffer, start, end )

  /** @type {Number} NEWLDR record size */
  this.size = 0x16
  // (80h-FEh, 00h-7Eh, FFh, 7Fh)
  // (optional, zero if not used)
  /** @type {Number} Physical drive and boot flag */
  this.physicalDrive = 0x00
  // (i.e. IBMBIO.LDR, 000000h if not used)
  /** @type {CHS} CHS address of LOADER boot sector or image file */
  this.firstCHS = new CHS()
  // (80h: default; 00h: always use DL; FFh: always use table entry)
  /** @type {Number} Allowed DL minimum, else take from partition table */
  this.minDL = 0x00
  /** @type {Number} LBA of LOADER boot sector or image file (optional) */
  this.firstLBA = 0x00000000
  // (default 0000h if not used, else 0024h or 01FDh)
  /** @type {Number} Patch offset of VBR boot unit */
  this.patchOffset = 0x0000
  /** @type {Number} Checksum (0000h if not used) */
  this.checksum = 0x0000
  // ("MSWIN4" for REAL/32, see also offset +0DAh,
  // corresponds with OEM label at offset +003h in VBRs)
  /** @type {String} OEM loader signature (optional) */
  this.signature = 'MSWIN4'

  if( Buffer.isBuffer( buffer ) ) {
    this.buffer = buffer.slice( start, end )
  }

}

/**
 * NEWLDR Record Prototype
 * @type {Object}
 * @ignore
 */
Record.prototype = {

  constructor: Record,

  get buffer() {

    var buffer = new Buffer( 30 )
        buffer.fill( 0 )

    buffer[0] = 0xEB // JPMS
    buffer.writeUInt8( this.size, 1 )
    buffer.write( 'NEWLDR', 2, 'ascii' )

    buffer.writeUInt8( this.physicalDrive, 8 )
    this.firstCHS.buffer.copy( buffer, 9 )
    buffer.writeUInt8( this.minDL, 12 )
    // 3 reserved bytes
    buffer.writeUInt32LE( this.firstLBA, 16 )
    buffer.writeUInt16LE( this.patchOffset, 20 )
    buffer.writeUInt16LE( this.checksum, 22 )
    buffer.write( this.signature, 24, 'ascii' )

    return buffer

  },

  set buffer( value ) {

    if( !Buffer.isBuffer( value ) )
      throw new TypeError( 'Value must be a Buffer' )

    this.size = value.readUInt8( 1 )
    // 6 byte "NEWLDR" signature
    this.physicalDrive   = value.readUInt8( 8 )
    this.firstCHS.buffer = value.slice( 9, 12 )
    this.minDL           = value.readUInt8( 12 )
    // 3 reserved bytes
    this.firstLBA        = value.readUInt32LE( 16 )
    this.patchOffset     = value.readUInt16LE( 20 )
    this.checksum        = value.readUInt16LE( 22 )
    this.signature       = value.toString( 'ascii', 24 )

  },

}

// Exports
module.exports = Record
