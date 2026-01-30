# Check for password protected files

This Spin application checks if a file is password protected or not.

Currently, the following file formats are supported:

- `.pdf`
- `.docx`
- `.xlsx`
- `.pptx`

To build the app, run `spin build`

## Testing

The app contains a bunch of sample documents in `./assets`. All password protected files, are protected with the `FooBar` password.

Start the application with `spin up` and use `curl` to test:

```bash

curl -i localhost:3000/assets/protected.pdf
curl -i localhost:3000/assets/protected.docx
curl -i localhost:3000/assets/protected.xlsx
curl -i localhost:3000/assets/protected.pptx

curl -i localhost:3000/assets/non-protected.pdf
curl -i localhost:3000/assets/non-protected.docx
curl -i localhost:3000/assets/non-protected.xlsx
curl -i localhost:3000/assets/non-protected.pptx
```
