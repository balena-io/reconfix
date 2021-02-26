/// <reference types="node" />
import { Disk } from 'file-disk';
import { TypedError } from 'typed-error';
export interface MBRPartition {
    offset: number;
    size: number;
    type: number;
    index: number;
}
export interface GPTPartition {
    offset: number;
    size: number;
    type: string;
    index: number;
    guid: string;
    name: string;
}
export declare type GetPartitionsResult = {
    type: 'mbr';
    partitions: MBRPartition[];
} | {
    type: 'gpt';
    partitions: GPTPartition[];
};
export declare class PartitionNotFound extends TypedError {
    constructor(partitionNumber: number);
}
/**
 * @summary Get information from a partition
 * @public
 * @function
 *
 * @param {String|Buffer|filedisk.Disk} image - image path or buffer or filedisk.Disk instance
 * @param {Object} number - partition number
 *
 * @returns {Promise<Object>} partition information
 *
 * @example
 * partitioninfo.get('foo/bar.img', 5)
 * .then (information) ->
 * 	console.log(information.offset)
 * 	console.log(information.size)
 * 	console.log(information.type)
 * 	console.log(information.index)
 */
export declare function get(disk: string | Buffer | Disk, partitionNumber: number): Promise<MBRPartition | GPTPartition>;
/**
 * @summary Read all partition tables from a disk image recursively.
 * @public
 * @function
 *
 * @description `getPartitions()` returns an Array.
 * `getPartitions(image)[N - 1]` may not be equal to `get(image, N)`
 * For example on a disk with no primary partitions and one extended partition
 * containing a logical one, `getPartitions(image)` would return an array of 2 partitions
 * (the extended then the logical one), `get(image, 1)` would return the extended
 * partition and `get(image, 5)` would return the logical partition. All other
 * numbers would throw an error.
 * Partition numbers for `get(image, N)` are like Linux's `/dev/sdaN`.
 *
 * The array returned by `getPartitions()` contains primary (or extended) partitions
 * first then the logical ones. This is true even if the extended partition is not the
 * last one of the disk image. Order will always be 1, [2, 3, 4, 5, 6, 7] even if
 * the logical partitions 5, 6 and 7 are physically contained in partiton 1, 2 or 3.
 *
 * @param {String|Buffer|filedisk.Disk} image - image path or buffer or filedisk.Disk instance
 * @param {Object} options
 * @param {Number} [options.offset=0] - where the first partition table will be read from, in bytes
 * @param {Boolean} [options.includeExtended=true] - whether to include extended partitions or not (only for MBR partition tables)
 * @param {Boolean} [options.getLogical=true] - whether to include logical partitions or not (only for MBR partition tables)
 *
 * @throws {Error} if there is no such partition
 *
 * @returns {Promise<Object>} partitions information
 *
 * @example
 * partitioninfo.getPartitions('foo/bar.img')
 * .then (information) ->
 * 	console.log(information.type)
 * 	for partition in information.partitions
 * 		console.log(partition.offset)
 * 		console.log(partition.size)
 * 		console.log(partition.type)
 * 		console.log(partition.index)
 */
export declare function getPartitions(disk: string | Buffer | Disk, { offset, includeExtended, getLogical, }?: {
    offset?: number;
    includeExtended?: boolean;
    getLogical?: boolean;
}): Promise<GetPartitionsResult>;
