package shared

import (
	"bytes"
	"encoding/json"
)

func NormalizeJSON(input []byte) ([]byte, error) {
	var output bytes.Buffer
	if err := json.Indent(&output, input, "", "  "); err != nil {
		return nil, err
	}
	return output.Bytes(), nil
}
