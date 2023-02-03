//! SSTable Module.
//!
//! Table([TableBlock], Footer)
//!     TableBlock(Block, CompressionType(1), CRC(4))
//!     Block = DataBlock | FilterBlock | MetaIndexBlock | IndexBlock
//!
//!     BLOCK_SIZE - default 4KiB
//!
//! Table([DataBlock], FilterBlock, MetaIndexBlock, IndexBlock, Footer)
//!
//! - DataBlock([Entry], [RestartPoint], restart_point_len(4))
//!     Entry(shared_key_len, unshared_key_len, value_len, unshared_key, value)
//!     RestartPoint = offset(4)
//!
//!     Block<Key, Value> -> BlockContents
//!
//! - FilterBlock([FilterData], [FilterOffset], filter_offsets_offset(4), base_lg(1))
//!
//!     Block<Key> -> BlockContents
//!
//! - MetaIndexBlock("filter.{name}", BlockHandle -> FilterBlock)
//!
//!     Block<Key, Value> -> BlockContents
//!         key = "filter.{name}"
//!         value = bytes(BlockHandle)
//!
//! - IndexBlock([(MaxKey, BlockHandle -> DataBlock]))
//!
//!     Block<Key, Value> -> BlockContents
//!         key = max key of indexed block (>= block lask key)
//!         value = bytes(BlockHandle)
//!
//! - Footer(MetaIndexBlockHandle -> MetaIndexBlock, IndexBlockHandle -> IndexBlock, Padding, Magic(8))
//!     MetaIndexBlockHandle = IndexBlockHandle = BlockHandle
//!
//! BlockHandle(offset(4), size(4))
//!
//! ==============================================================================================
//!
//! DataBlock = MetaIndexBlock = IndexBlock = Block<Key, Value> <- Block, BlockIter, BlockBuilder
//! FilterBlock = Block<Key> <- FilterBlock, FilterBuilder
//!
//! ==============================================================================================
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
