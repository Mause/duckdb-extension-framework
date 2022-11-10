use crate::duckly::{
    duckdb_create_data_chunk, duckdb_data_chunk, duckdb_data_chunk_get_column_count,
    duckdb_data_chunk_get_size, duckdb_data_chunk_get_vector, duckdb_data_chunk_reset,
    duckdb_data_chunk_set_size, duckdb_destroy_data_chunk, duckdb_logical_type, idx_t,
};
use crate::{LogicalType, Vector};

/// A Data Chunk represents a set of vectors.
///
/// The data chunk class is the intermediate representation used by the
/// execution engine of DuckDB. It effectively represents a subset of a relation.
/// It holds a set of vectors that all have the same length.
///
/// DataChunk is initialized using the DataChunk::Initialize function by
/// providing it with a vector of TypeIds for the Vector members. By default,
/// this function will also allocate a chunk of memory in the DataChunk for the
/// vectors and all the vectors will be referencing vectors to the data owned by
/// the chunk. The reason for this behavior is that the underlying vectors can
/// become referencing vectors to other chunks as well (i.e. in the case an
/// operator does not alter the data, such as a Filter operator which only adds a
/// selection vector).
///
/// In addition to holding the data of the vectors, the DataChunk also owns the
/// selection vector that underlying vectors can point to.
#[derive(Debug)]
pub struct DataChunk {
    ptr: duckdb_data_chunk,
    owned: bool,
}

impl DataChunk {
    /// Creates an empty DataChunk with the specified set of types.
    ///
    /// # Arguments
    /// - `types`: An array of types of the data chunk.
    pub fn new(types: Vec<LogicalType>) -> Self {
        let types: Vec<duckdb_logical_type> = types.iter().map(|x| x.typ).collect();
        let mut types = types.into_boxed_slice();

        let ptr = unsafe {
            duckdb_create_data_chunk(types.as_mut_ptr(), types.len().try_into().unwrap())
        };

        Self { ptr, owned: true }
    }

    /// Retrieves the vector at the specified column index in the data chunk.
    ///
    /// The pointer to the vector is valid for as long as the chunk is alive.
    /// It does NOT need to be destroyed.
    ///
    pub fn get_vector<T>(&self, column_index: idx_t) -> Vector<T> {
        Vector::from(unsafe { duckdb_data_chunk_get_vector(self.ptr, column_index) })
    }
    /// Sets the current number of tuples in a data chunk.
    pub fn set_size(&self, size: idx_t) {
        unsafe { duckdb_data_chunk_set_size(self.ptr, size) };
    }
    /// Resets a data chunk, clearing the validity masks and setting the cardinality of the data chunk to 0.
    pub fn reset(&self) {
        unsafe { duckdb_data_chunk_reset(self.ptr) }
    }
    /// Retrieves the number of columns in a data chunk.
    pub fn get_column_count(&self) -> idx_t {
        unsafe { duckdb_data_chunk_get_column_count(self.ptr) }
    }
    /// Retrieves the current number of tuples in a data chunk.
    pub fn get_size(&self) -> idx_t {
        unsafe { duckdb_data_chunk_get_size(self.ptr) }
    }
}

impl From<duckdb_data_chunk> for DataChunk {
    fn from(ptr: duckdb_data_chunk) -> Self {
        Self { ptr, owned: false }
    }
}

impl Drop for DataChunk {
    fn drop(&mut self) {
        if self.owned {
            unsafe { duckdb_destroy_data_chunk(&mut self.ptr) };
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{DataChunk, LogicalType};

    #[test]
    fn test_data_chunk_construction() {
        let dc = DataChunk::new(vec![LogicalType::new(
            crate::constants::LogicalTypeId::Integer,
        )]);

        assert_eq!(dc.get_column_count(), 1);

        drop(dc);
    }
}
