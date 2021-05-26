#!/usr/bin/perl
use strict;

# usage: add_copyright file... dir

my $dir = pop @ARGV;
if (not -d $dir) {
	print "$dir is not a directory\n";
	exit 1;
}

my $copyright = `cat copyright`;
my $copyright_oneline = `grep '(C)' copyright`;
$copyright_oneline =~ s/^\s*\*\s*//;
$copyright_oneline =~ s/[\r\n]//g;

foreach my $in (@ARGV) {
	my $out = "$dir/$in";
	if (not open IN, $in) {
		print "cannot open $in: $!\n";
		next;
	}
	if (not open OUT, ">$out") {
		print "cannot create $out: $!\n";
		close IN;
		next
	}
	while (<IN>) {
		if (/\@\@copyright\@\@/) {
			print OUT $copyright;
		} elsif (/\@\@copyright_oneline\@\@/) {
			s/\@\@copyright_oneline\@\@/$copyright_oneline/;
			print OUT;
		} else {
			print OUT;
		}
	}
	close OUT;
	close IN;
	my $mode = (-x $in) ? 0555 : 0444;
	chmod $mode, $out;
}
