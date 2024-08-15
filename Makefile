.POSIX:

flatbuffers:
	flatc --rust --filename-suffix '' -o src/flatbuffers flatbuffers/*.fbs

.PHONY: flatbuffers
