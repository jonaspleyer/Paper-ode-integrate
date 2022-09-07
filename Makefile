CC:=latexmk
SRCDIR:=src
BIBDIR:=src/bibliography
MEDIADIR:=src/media
ODIR:=out
TARGET:=main
OPTIONS:=-pdf -shell-escape
TEX_FILES:=$(wildcard $(SRCDIR)/*.tex)
COMPILATION_FILES:=$(filter-out src/$(TARGET)_expanded.tex, $(TEX_FILES))


ZIP_TARGETS:= README.md $(BIBDIR)/cited.bib $(SRCDIR)/$(TARGET)_expanded.tex $(ODIR)/$(TARGET).pdf

all:
	$(CC) $(OPTIONS) --output-directory=$(CURDIR)/$(ODIR) -cd $(CURDIR)/$(SRCDIR)/$(TARGET).tex

clean: clean_cite
	-rm -r $(ODIR)/*
	-rm $(SRCDIR)/$(TARGET)_expanded.tex
	-rm $(CURDIR)/release.zip

clean_cite:
	-rm -r $(BIBDIR)/cited.bib*

bibexport: all
	bibexport -r $(BIBDIR)/Paper-ode-integrate.bib -o $(BIBDIR)/cited.bib $(ODIR)/$(TARGET).aux

latexpand:
	cd $(SRCDIR); latexpand $(TARGET).tex > $(TARGET)_expanded.tex

fresh: clean clean_cite all

examine:
	pplatex -i $(ODIR)/$(TARGET).log

wordcount:
	texcount $(COMPILATION_FILES)

citecount: clean_cite bibexport
	cat src/bibliography/cited.bib | grep -o ^@ | wc -l

zip: fresh bibexport latexpand
	zip $(CURDIR)/release.zip $(ZIP_TARGETS)
	printf "@ $(BIBDIR)/cited.bib\n@=$(BIBDIR)/Paper-ode-integrate.bib\n" | zipnote -w release.zip
