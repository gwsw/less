#! /usr/bin/env perl
use strict;

my $state = 0;
my $def;
my $params;

while (<>) {
	if (/^\tpublic\s+(.*)/) {
		$def = "public $1";
		$state = 1;
		$params = 0;
	} elsif ($state == 1 and /(\w+)\s*\((.*)\)/) {
		$def .= " $1 LESSPARAMS (($2))";
		$state = 2;
	} elsif ($state == 2) {
		print "$def;\n";
		$state = 0;
	}
}
