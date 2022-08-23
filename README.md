# bencode-parser
A [bencode](https://en.wikipedia.org/wiki/Bencode) parser written in Rust.

## Overview
Bencode is the encoding used by the [Bittorrent specification](https://wiki.theory.org/BitTorrentSpecification#Bencoding) to store loosely-structed metadata. <br>
It supports the following data types: <br>
* Integers: `i<int32>e`
* Strings: `<size>:<string>`
* Lists: `l<elements>e`
* Dictionaries: `d<elements>e>` where `elements` is structured as `key:value`
