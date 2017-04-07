#! /usr/bin/perl
use strict;

my $state = 0;
my $def;

while (<>) {
	if (/^\tpublic\s+(.*)/) {
		$def = "public $1";
		$state = 1;
	} elsif ($state == 1 and /(\w+)\s*\(/) {
		$def .= " $1 LESSPARAMS((";
		$state = 2;
	} elsif ($state == 2) {
		if (/^{/) {
			print "$def));\n";
			$state = 0;
		} elsif (/^\s*([^;]*)/) {
			$def .= ', ' if substr($def,-1) ne '(';
			$def .= $1;
		}
	}
}
