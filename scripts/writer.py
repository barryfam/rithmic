from string import Template
from textwrap import indent
from typing import TextIO

class AtTemplate(Template):
    delimiter = '@'

class Writer:
    def __init__(self, mapping: dict, file: TextIO):
        self.mapping = mapping
        self.file = file

    def __call__(self, template: str='', tabs=0, newlines=1):
        template = indent(template.rstrip(' ').strip('\n'), ' '*4 * tabs)
        print(AtTemplate(template).substitute(self.mapping), end='\n' * newlines, file=self.file)
