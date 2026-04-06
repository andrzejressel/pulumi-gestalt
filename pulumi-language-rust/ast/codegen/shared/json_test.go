package shared

import "testing"

func TestNormalizeJSON(t *testing.T) {
	t.Parallel()

	t.Run("formats minified json", func(t *testing.T) {
		t.Parallel()

		input := []byte(`{"a":1,"b":{"c":true}}`)
		expected := "{\n  \"a\": 1,\n  \"b\": {\n    \"c\": true\n  }\n}"

		output, err := NormalizeJSON(input)
		if err != nil {
			t.Fatalf("NormalizeJSON returned error: %v", err)
		}
		if string(output) != expected {
			t.Fatalf("unexpected output:\n%s", string(output))
		}
	})

	t.Run("handles empty object", func(t *testing.T) {
		t.Parallel()

		output, err := NormalizeJSON([]byte(`{}`))
		if err != nil {
			t.Fatalf("NormalizeJSON returned error: %v", err)
		}
		if string(output) != "{}" {
			t.Fatalf("unexpected output: %s", string(output))
		}
	})

	t.Run("returns error for invalid json", func(t *testing.T) {
		t.Parallel()

		_, err := NormalizeJSON([]byte(`{"a":`))
		if err == nil {
			t.Fatal("expected error, got nil")
		}
	})
}
