# SSTable Format

> Reference from:
> - <https://github.com/google/leveldb/blob/main/doc/table_format.md>
> - <https://leveldb-handbook.readthedocs.io/zh/latest/basic.html>


## 1. Table

* Table = ([TableBlock], Footer)
* - TableBlock = (Block, CompressionType(1), CRC(4))
* - Block = DataBlock | FilterBlock | MetaIndexBlock | IndexBlock
* - CompressionType = None | Snap

> BLOCK_SIZE, default 4KiB.

* Table = ([DataBlock'], FilterBlock', MetaIndexBlock', IndexBlock', Footer)
* - DataBlock' = (DataBlock, CompressionType, CRC)
* - FilterBlock' = (FilterBlock, CompressionType, CRC)
* - MetaIndexBlock' = (MetaIndexBlock, CompressionType, CRC)
* - IndexBlock' = (IndexBlock, CompressionType, CRC)

### 1) DataBlock

* DataBlock = [Entry] + [RestartPoint] + restart_point_len(4)
* - Entry = shared_key_len + unshared_key_len + value_len + unshared_key + value
* - RestartPoint = Offset(4)

> BlockContents = Block<Key, Value>


### 2) FilterBlock

* FilterBlock = ([FilterData], [FilterOffset], filter_offset_offset(4), base_lg2(1))

> BlockContents = Block<Key>

### 3) MetaIndexBlock

* MetaIndexBlock = ("filter.{name}", BlockHandle -> FilterBlock)
* - BlockHandle = (Offset, Size)

> BlockContents = Block<Key, Value>
> - key = "filter.{name}"
> - value = bytes(BlockHandle)

### 4) IndexBlock

* IndexBlock = ([(MaxKey, BlockHandle -> DataBlock)])

> BlockContents = Block<Key, Value>
> - key = <max key of indexed block (>= block lask key)>
> - value = bytes(BlockHandle)

### 5) Footer

* Footer = (BlockHandle -> MetaIndexBlock, BlockHandle -> IndexBlock, Padding, Magic(8))

### 6) BlockHandle

* BlockHandle = (Offset(4), Size(4))

### 7) Block

* DataBlock = MetaIndexBlock = IndexBlock = Block<Key, Value>

> Block, BlockIter, BlockBuilder

* FilterBlock = Block<Key>

> FilterBlock, FilterBlockIter, FilterBlockBuilder



//!
//! TableBuilder
//!     .DataBlockBuilder       -> BlockBuilder
//!     .FilterBlockBuilder     -> FilterBlockBuilder
//!     .MetaIndexBlockBuilder  -> BlockBuilder
//!     .IndexBlockBuilder      -> BlockBuilder
//!     .Footer<Builder>
//!
//! Table<Reader>
//!     .File(Size)
//!     .Cache <- Options
//!     .Footer<Reader>
//!     .IndexBlock<Reader> -> Block<Reader> -> Cache
//!     .FilterBlock<Reader>
//!     .iter -> TableIterator
//!     .get(Key) -> Value
//!
//! TableIterator -> DBIterator
//!     .BlockIterator -> DBIterator
//!
//! BlockBuilder
//!     .buffer -> BlockContents
//!     .restarts
//!     .last_key
//!     .restarts_counter
//!     .counter
//!     <- Options.block_restart_interval: default(16)
//!     <- Options.cmp
//!
//! FilterBuilder
//!     .policy: BoxedFilterPolicy <- Options.policy_filter
//!     .filters -> BlockContents
//!     .filter_offsets
//!     // reset on every start_block().
//!     .keys
//!     .key_offsets
//!
//! FilterBlock<Reader>
//!     .policy
//!     .block -> BlockContents
//!     .offsets_offset
//!     .filter_base_lg2
//!     .key_may_match(block_index, key) -> bool
//!
//! Block<Reader>
//!     .block -> BlockContents
//!     .iter() -> BlockIterator
//!
//! BlockIterator
//!     .block
//!     <- Options.cmp
//!
