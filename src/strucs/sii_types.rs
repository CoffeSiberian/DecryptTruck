pub enum _DataTypeIdFormatHex {
    // string, char is 8 bits, not null terminated
    UTF8String = 0x01,
    // array of type 0x01
    ArrayOfUTF8String = 0x02,
    // string stored as a 64bit number, only lower 63 bits matter
    EncodedString = 0x03,
    // array of type 0x03
    ArrayOfEncodedString = 0x04,
    // 4 byte float
    Single = 0x05,
    // array of 4 byte float
    ArrayOfSingle = 0x06,
    // 2 4 byte floats
    VectorOf2Single = 0x07,
    // Array of vectors of 2 4 byte floats (array of type 0x07) //EXPERIMENTAL
    ArrayOfVectorOf2Single = 0x08,
    // 3 4 byte floats
    VectorOf3Single = 0x09,
    // Array of vectors of 3 4 byte floats (array of type 0x09)
    ArrayOfVectorOf3Single = 0x0A,
    // 3 4 byte signed integers
    VectorOf3Int32 = 0x11,
    // Array of vectors of 3 4 byte signed integers (array of type 0x11)
    ArrayOfVectorOf3Int32 = 0x12,
    // vector of 4 4 byte floats
    VectorOf4Single = 0x17,
    // Array of vectors of 4 4 byte floats (array of type 0x17)
    ArrayOfVectorOf4Single = 0x18,
    // Vector of 8 4 byte floats (format 1 is 7)
    VectorOf8Single = 0x19,
    // Array of Vectors of 8 4 byte floats (array of 0x19 - format 1 is 7)
    ArrayOfVectorOf8Single = 0x1A,
    // Signed 32-bit integer
    Int32 = 0x25,
    // Array of Int32
    ArrayOfInt32 = 0x26,
    // Unsigned 32-bit integer
    UInt32 = 0x27,
    // Array of UInt32
    ArrayOfUInt32 = 0x28,
    // signed 16 bit integer //EXPERIMENTAL
    Int16 = 0x29,
    // Array of 16 bit signed integers (array of type 0x29) //EXPERIMENTAL
    ArrayOfInt16 = 0x2A,
    // Unsigned 16 bit integer
    UInt16 = 0x2B,
    // Array of UInt16
    ArrayOfUInt16 = 0x2C,
    // UInt32 (same as 0x27)
    UInt32Type2 = 0x2F,
    // 64 bit signed integer
    Int64 = 0x31,
    // Array of 64-bit signed integer (array of type 0x31) //EXPERIMENTAL
    ArrayOfInt64 = 0x32,
    // 64 bit unsigned integer
    UInt64 = 0x33,
    // Array of 64 bit unsigned integers (array of type 0x33)
    ArrayOfUInt64 = 0x34,
    // 8 bit bool - 0 = false, any other = true
    ByteBool = 0x35,
    // Array of 8 bit bools (array of type 0x35)
    ArrayOfByteBool = 0x36,
    // Original String
    OrdinalString = 0x37,
    // Id complex type
    Id = 0x39,
    // Array of Id
    ArrayOfIdA = 0x3A,
    // Array of Id
    ArrayOfIdC = 0x3C,
    // Id Complex type
    IdType2 = 0x3B,
    // Id complex type
    IdType3 = 0x3D,
    // Array of Id //EXPERIMENTAL
    ArrayOfIdE = 0x3E,
    // Vector of 2 4 byte UInt32
    ArrayOfVectorOf2Int32 = 0x41,
}

pub enum DataTypeIdFormat {
    // string, char is 8 bits, not null terminated
    UTF8String = 1,
    // array of type 0x01
    ArrayOfUTF8String = 2,
    // string stored as a 64bit number, only lower 63 bits matter
    EncodedString = 3,
    // array of type 0x03
    ArrayOfEncodedString = 4,
    // 4 byte float
    Single = 5,
    // array of 4 byte float
    ArrayOfSingle = 6,
    // 2 4 byte floats
    VectorOf2Single = 7,
    // Array of vectors of 2 4 byte floats (array of type 0x07) //EXPERIMENTAL
    ArrayOfVectorOf2Single = 8,
    // 3 4 byte floats
    VectorOf3Single = 9,
    // Array of vectors of 3 4 byte floats (array of type 0x09)
    ArrayOfVectorOf3Single = 10,
    // 3 4 byte signed integers
    VectorOf3Int32 = 17,
    // Array of vectors of 3 4 byte signed integers (array of type 0x11)
    ArrayOfVectorOf3Int32 = 18,
    // vector of 4 4 byte floats
    VectorOf4Single = 23,
    // Array of vectors of 4 4 byte floats (array of type 0x17)
    ArrayOfVectorOf4Single = 24,
    // Vector of 8 4 byte floats (format 1 is 7)
    VectorOf8Single = 25,
    // Array of Vectors of 8 4 byte floats (array of 0x19 - format 1 is 7)
    ArrayOfVectorOf8Single = 26,
    // Signed 32-bit integer
    Int32 = 37,
    // Array of Int32
    ArrayOfInt32 = 38,
    // Unsigned 32-bit integer
    UInt32 = 39,
    // Array of UInt32
    ArrayOfUInt32 = 40,
    // signed 16 bit integer //EXPERIMENTAL
    Int16 = 41,
    // Array of 16 bit signed integers (array of type 0x29) //EXPERIMENTAL
    ArrayOfInt16 = 42,
    // Unsigned 16 bit integer
    UInt16 = 43,
    // Array of UInt16
    ArrayOfUInt16 = 44,
    // UInt32 (same as 0x27)
    UInt32Type2 = 47,
    // 64 bit signed integer
    Int64 = 49,
    // Array of 64-bit signed integer (array of type 0x31) //EXPERIMENTAL
    ArrayOfInt64 = 50,
    // 64 bit unsigned integer
    UInt64 = 51,
    // Array of 64 bit unsigned integers (array of type 0x33)
    ArrayOfUInt64 = 52,
    // 8 bit bool - 0 = false, any other = true
    ByteBool = 53,
    // Array of 8 bit bools (array of type 0x35)
    ArrayOfByteBool = 54,
    // Original String
    OrdinalString = 55,
    // Id complex type
    Id = 57,
    // Array of Id
    ArrayOfIdA = 58,
    // Array of Id
    ArrayOfIdC = 60,
    // Id Complex type
    IdType2 = 59,
    // Id complex type
    IdType3 = 61,
    // Array of Id //EXPERIMENTAL
    ArrayOfIdE = 62,
    // Vector of 2 4 byte UInt32
    ArrayOfVectorOf2Int32 = 65,
}
