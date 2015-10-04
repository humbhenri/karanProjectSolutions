package invertedIndex

import (
	"io/ioutil"
	"os"
	"testing"
)

func TestAddFile(t *testing.T) {
	file := createFileWithText("example")
	index := New()
	index.Parse(file)
	result := index.Get("example")[0]
	if result.File != file {
		t.Errorf("file name should be %s\n", file)
	}
	if result.Line != 1 {
		t.Errorf("line should be %d\n", 1)
	}
	if result.Index != 1 {
		t.Errorf("index should be %d\n but was %d\n", 1, result.Index)
	}
	defer deleteFile(file)
}

func TestWithMoreThanOneFile(t *testing.T) {
	file1 := createFileWithText("text1")
	file2 := createFileWithText("text1")
	defer deleteFile(file1)
	defer deleteFile(file2)

	index := New()
	index.Parse(file1)
	index.Parse(file2)
	results := index.Get("text1")
	result1 := results[0]
	result2 := results[1]
	if result1.File != file1 {
		t.Errorf("First file should be %s\n", file1)
	}
	if result2.File != file2 {
		t.Errorf("Second file should be %s\n", file2)
	}
}

func TestWithMoreWords(t *testing.T) {
	file := createFileWithText("alea jacta est")
	defer deleteFile(file)

	index := New()
	index.Parse(file)
	results := index.Get("est")
	if len(results) != 1 {
		t.Errorf("There should be only one result but it was %d\n", len(results))
	}
	if results[0].File != file {
		t.Errorf("file name should be %s\n", file)
	}
	if results[0].Index != 12 {
		t.Errorf("index should be %d\n but was %d\n", 1, results[0].Index)
	}
}

func TestWordNotFound(t *testing.T) {
	file := createFileWithText("alea jacta est")
	defer deleteFile(file)

	index := New()
	results := index.Get("banana")
	if len(results) != 0 {
		t.Error("should be no results")
	}
}

func createFileWithText(text string) string {
	file, err := ioutil.TempFile("", "gotesting")
	if err != nil {
		panic(err)
	}
	file.WriteString(text)
	defer file.Close()
	return file.Name()
}

func deleteFile(filename string) {
	if err := os.Remove(filename); err != nil {
		panic(err)
	}
}
