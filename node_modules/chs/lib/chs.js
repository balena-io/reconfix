/**
 * Cylinder-Head-Sector Address
 * @constructor
 * @param {(Number|Buffer)} [cylinder=1023]
 * @param {Number} [head=254]
 * @param {Number} [sector=63]
 */
function CHS( cylinder, head, sector ) {

  if( !(this instanceof CHS) )
    return new CHS( cylinder, head, sector )

  if( Buffer.isBuffer( cylinder ) )
    return CHS.fromBuffer( cylinder )

  /** @type {Number} Cylinder */
  this.cylinder = cylinder & 0x03FF
  /** @type {Number} Head */
  this.head = head & 0xFF
  /** @type {Number} Sector */
  this.sector = sector & 0x3F

}

/**
 * Create a CHS Address from a given buffer
 * @param {Buffer} buffer
 * @param {Number} [offset=0]
 * @return {CHS}
 */
CHS.fromBuffer = function( buffer, offset ) {
  return new CHS().parse( buffer, offset )
}

/**
 * Create a CHS Address from a Logical Block Address (LBA)
 * @param {Number} lba Logical Block Address
 * @param {Number} hpt Heads per Track
 * @param {Number} spt Sectors per Track
 * @return {CHS}
 */
CHS.fromLBA = function( lba, hpt, spt ) {
  return new CHS().setLBA( lba, hpt, spt )
}

/**
 * CHS prototype
 * @type {Object}
 * @ignore
 */
CHS.prototype = {

  constructor: CHS,

  /** @type {Buffer} Get/set values from/to a Buffer */
  set buffer( value ) { return this.parse( value ) },
  get buffer() { return this.toBuffer() },

  /**
   * Set CHS to a Logical Block Address (LBA)
   * @param {Number} lba Logical Block Address
   * @param {Number} hpt Heads per Track
   * @param {Number} spt Sectors per Track
   * @return {CHS}
   */
  setLBA: function( lba, hpt, spt ) {
    this.cylinder = ( lba / ( spt * hpt )) | 0
    this.head     = ( lba / spt ) % hpt
    this.sector   = ( lba % spt ) + 1
    return this
  },

  /**
   * Get the Logical Block Address (LBA)
   * corresponding to the given disk geometry
   * @param {Number} hpt Heads per Track
   * @param {Number} spt Sectors per Track
   * @return {Number} lba
   */
  getLBA: function( hpt, spt ) {
    return ( this.cylinder * hpt + this.head ) *
      spt + ( this.sector - 1 )
  },

  /**
   * @see #getLBA()
   * @param {Number} hpt Heads per Track
   * @param {Number} spt Sectors per Track
   * @return {Number} lba
   */
  toLBA: function( hpt, spt ) {
    return this.getLBA( hpt, spt )
  },

  /**
   * Clone the CHS Address
   * @return {CHS}
   */
  clone: function() {
    return new CHS( this.cylinder, this.head, this.sector )
  },

  /**
   * Copy this address to a target address
   * @param {CHS} target
   * @return {CHS}
   */
  copy: function( target ) {

    target.cylinder = this.cylinder
    target.head = this.head
    target.sector = this.sector

    return target

  },

  /**
   * Parse a given Buffer
   * @param {Buffer} buffer
   * @param {Number} [offset=0]
   * @return {CHS}
   */
  parse: function( buffer, offset ) {

    if( !Buffer.isBuffer( buffer ) )
      throw new TypeError( 'Value must be a buffer' )

    offset = offset || 0

    return this.fromNumber( buffer.readUIntLE( offset, 3 ) )

  },

  /**
   * Write the CHS address to a given buffer
   * @param {Buffer} buffer
   * @param {Number} [offset=0]
   * @returns {Buffer}
   */
  write: function( buffer, offset ) {
    offset = offset || 0
    buffer.writeUIntLE( this.toNumber(), offset, 3 )
    return buffer
  },

  /**
   * Create a Buffer representation of the CHS Address
   * @return {Buffer}
   */
  toBuffer: function() {
    return this.write( Buffer.alloc( 3 ) )
  },

  /**
   * Set the CHS address from its 24bit integer value
   * @param {Number} value
   * @returns {CHS}
   */
  fromNumber: function( value ) {

    // The head is in the low 8 bits;
    // 11111111 00000000 00000000
    this.head = ( value & 0xFF0000 ) >>> 16

    // The cylinder in bits 8-16 + 16-18;
    // 00000000 11111111 11000000
    this.cylinder = ( value & 0x00FFC0 ) >>> 6

    // The sector in bits 18-24;
    // 00000000 00000000 00111111
    this.sector = ( value & 0x00003F )

    return this

  },

  /**
   * Get the 24bit integer value of the CHS address
   * @return {Number}
   */
  toNumber: function() {

    // 11111111 00000000 00000000
    var value = ( this.head & 0xFF ) << 16

    // 00000000 11111111 11000000
    value = value | (( this.cylinder << 6 ) & 0xFFC0 )

    // 00000000 00000000 00111111
    value = value | ( this.sector & 0x3F )

    return value

  },

}

// Exports
module.exports = CHS
