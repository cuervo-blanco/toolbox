```plaintext
mdconvert - A Minimal Markdown to Document Conversion Tool

USAGE:
    mdconvert [SOURCE FILE] [FORMAT]

DESCRIPTION:
    Converts a Markdown (.md) file to a specified document format.
    Supported formats include:
        - docx (Microsoft Word)
        - pdf (Portable Document Format)
        - html (HyperText Markup Language)
        - odt (OpenDocument Text)
        - rtf (Rich Text Format)
        - txt (Plain Text)
        - epub (Electronic Publication)

ARGUMENTS:
    SOURCE FILE    The path to the Markdown (.md) file to be converted.
    FORMAT         The target format for the output file (e.g., "docx").

EXAMPLES:
    # Convert a file from .md to .docx
    mdconvert readme.md docx

    # Convert a file from .md to .pdf
    mdconvert guide.md pdf

OUTPUT:
    On success, creates a new file with the specified format in the same
    directory as the source file. The output file name is generated from
    the source file name with the target format as the new extension.

NOTES:
    - Ensure Pandoc is installed and accessible in your PATH.
    - Unsupported formats will return an error message.

EXIT CODES:
    0   Success
    1   Error (e.g., unsupported format or missing arguments)

```
