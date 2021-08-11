pub enum DataType {
    SZSE_L2_Status = 0x00202007,
    SZSE_L2_Order = 0x00202006,
    SZSE_L2_Index = 0x00202003,
    SZSE_L2_Transaction = 0x00202002,
    SZSE_L2_Quotation = 0x00202001,

    SSE_L2_Status = 0x00102000,
    SSE_L2_Order = 0x00102006,
    SSE_L2_Index = 0x00102003,
    SSE_L2_Transaction = 0x00102002,
    SSE_L2_Quotation = 0x00102001,
    SSE_L2_Auction = 0x00102004,
    SSE_L2_Overview = 0x00102005,
}