#include <stdlib.h>
#include <string.h>
#include "density_api.h"

int main() {
    char* text = "This is a simple example";// on how to use the simple Density API.";
    uint64_t text_length = (uint64_t)strlen(text);

    // Determine safe buffer sizes
    uint_fast64_t compress_safe_size = density_compress_safe_size(text_length);
    uint_fast64_t decompress_safe_size = density_decompress_safe_size(text_length);

    // Allocate required memory
    uint8_t *outCompressed   = malloc(compress_safe_size * sizeof(char));
    uint8_t *outDecompressed = malloc(decompress_safe_size * sizeof(char));
    density_processing_result result;

    // Compress
    result = density_compress(text, text_length, outCompressed, compress_safe_size, DENSITY_ALGORITHM_CHAMELEON);
    if(!result.state)
        printf("Compressed %llu bytes to %llu bytes\n", result.bytesRead, result.bytesWritten);

    // Decompress
    result = density_decompress(outCompressed, result.bytesWritten, outDecompressed, decompress_safe_size);
    if(!result.state)
        printf("Decompressed %llu bytes to %llu bytes\n", result.bytesRead, result.bytesWritten);

    // Free memory_allocated
    free(outCompressed);
    free(outDecompressed);
}