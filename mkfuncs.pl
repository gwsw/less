#! /usr/bin/env perl
use strict;
use Getopt::Std;

my $state = 0;
my $def;
my $params;

my %opt;
die "usage: mkfuncs.pl [-p]\n" if not getopts('p', \%opt);
my $passthru = $opt{p};
while (<>) {
	my $match = $passthru ? /^\t(public|static)\s+([^;]+)$/ : /^\t(public)\s+([^;]+)$/;
	if ($match) {
		$def = "$1 $2";
		$state = 1;
		$params = 0;
	} elsif ($state == 1 and /(\w+)\s*\(/) {
		$def .= " $1";
		$def .= " LESSPARAMS (" if not $passthru;
		$def .= "(";
		$state = 2;
	} elsif ($state == 2) {
		if (/^{/) {
			if (not $params) {
				$def .= ($passthru ? 'void' : 'void');
			}
			print $def;
			print ")" if not $passthru;
			print ")\n{\n";
			$state = 0;
		} elsif (/^\s*([^;]*)/) {
			$def .= ', ' if substr($def,-1) ne '(';
			$def .= $1;
			$params = 1;
		}
	} else {
		print if $passthru;
	}
}
