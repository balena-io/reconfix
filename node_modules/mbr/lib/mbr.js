/**
 * Master Boot Record (MBR)
 * @class
 * @constructs MBR
 * @param {Buffer} [buffer]
 * @param {Number} [start]
 * @param {Number} [end]
 */
function MBR( buffer, start, end ) {

  if( !(this instanceof MBR) || this.format === 'MBR' ) {
    return MBR.parse( buffer, start, end )
  }

  /** @type {MBR.Partition[]} Partition table */
  this.partitions = []
  /** @type {MBR.Code[]} Bootloader code */
  this.code = []

  for( var i = 0; i < this.partitionEntries; i++ ) {
    this.partitions.push( new MBR.Partition() )
  }

  if( Buffer.isBuffer( buffer ) ) {
    this.parse( buffer, start, end )
  }

}

/**
 * Default partition table offset
 * @const {Number}
 */
MBR.TABLE_OFFSET = 0x1BE

/**
 * Default number of partition entries
 * @const {Number}
 */
MBR.PARTITION_ENTRIES = 4

MBR.Partition = require( './partition' )
MBR.Code = require( './code' )

/**
 * Parses a buffer into an instance of MBR
 * @param {Buffer} buffer
 * @param {Number} [start]
 * @param {Number} [end]
 * @returns {MBR}
 */
MBR.parse = function( buffer, start, end ) {
  var buffer = buffer.slice( start, end )
  var format = MBR.detectFormat( buffer )
  return new MBR[ format ]( buffer )
}

/**
 * Detects the MBR format of a given buffer
 * @param {Buffer} buffer
 * @returns {String} format
 */
MBR.detectFormat = function( buffer ) {

  if( !Buffer.isBuffer( buffer ) )
    throw new TypeError( 'Argument must be a Buffer' )

  if( buffer.length < 512 )
    throw new Error( 'Buffer too small (must be at least 512 bytes)' )

  // TODO: Move this into it's own static method (?)
  if( buffer.readUInt16LE( 0x1FE ) !== 0xAA55 ) {
    throw new SyntaxError(
      'Invalid MBR boot signature. Expected 0xAA55, ' +
      'but saw 0x' + buffer.readUInt16LE( 0x1FE )
        .toString( 16 ).toUpperCase()
    )
  }

  if( buffer[ 0x17C ] === 0x5A && buffer[ 0x17D ] === 0xA5 ) {
    return 'AST' // AST/NEC
  } else if( buffer[ 0x0FC ] === 0xAA && buffer[ 0x0FD ] === 0x55 ) {
    return 'DM' // Disk Manager
  } else if( buffer.readUIntBE( 0x02, 6 ) === 0x4E45574C4452 ) { // 'NEWLDR'
    return 'NEWLDR'
  } else if( buffer[ 0x1AC ] === 0x78 && buffer[ 0x1AD ] === 0x56 ) {
    return 'AAP'
  } else if( buffer[ 0x0DA ] === 0x00 && buffer[ 0x0DB ] === 0x00 ) {
    return 'MODERN'
  } else {
    return 'CLASSIC'
  }

}

/**
 * Creates a blank buffer with an MBR signature
 * @returns {Buffer}
 */
MBR.createBuffer = function() {
  // 512 byte buffer
  var buffer = new Buffer( 0x200 )
  // Zero it
  buffer.fill( 0 )
  // Write MBR signature
  buffer[ 0x1FE ] = 0x55
  buffer[ 0x1FF ] = 0xAA

  return buffer

}

/**
 * Determines if a given partition is an extended partition
 * @param {MBR.Partition} partition
 * @returns {Boolean}
 */
MBR.isExtendedPartition = function( partition ) {
  return MBR.Partition.isExtended( partition.type )
}

/**
 * MBR prototype
 * @type {Object}
 * @ignore
 */
MBR.prototype = {

  constructor: MBR,

  get buffer() {
    throw new Error( 'Not implemented' )
  },

  set buffer( value ) {
    throw new Error( 'Not implemented' )
  },

  /**
   * MBR format
   * @type {String}
   * @readOnly
   */
  get format() {
    return this.constructor.name.toUpperCase()
  },

  /**
   * Partition table offset
   * @type {Number}
   * @readOnly
   */
  get tableOffset() {
    return this.constructor.TABLE_OFFSET
  },

  /**
   * Number of partition entries
   * @type {Number}
   * @readOnly
   */
  get partitionEntries() {
    return this.constructor.PARTITION_ENTRIES
  },

  /**
   * Parse a Buffer
   * @param {Buffer} buffer
   * @param {Number} [start]
   * @param {Number} [end]
   * @returns {MBR}
   */
  parse: function( buffer, start, end ) {
    this.buffer = buffer.slice( start, end )
  },

  /**
   * Get the EFI system partition if available
   * @returns {MBR.Partition|null}
   */
  getEFIPart: function() {

    var i = 0
    var part = null

    for( var i = 0; i < this.partitions.length; i++ ) {
      part = this.partitions[i]
      if( part.type === 0xEE || part.type === 0xEF ) {
        return part
      }
    }

    return null

  },

}

// Exports
module.exports = MBR

MBR.CLASSIC = require( './format/classic' )
MBR.MODERN = require( './format/modern' )
MBR.AAP = require( './format/aap' )
MBR.NEWLDR = require( './format/newldr' )
MBR.AST = require( './format/ast' )
MBR.DM = require( './format/dm' )
