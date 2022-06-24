# Scuffed Note

An [Ali Express](https://aliexpress.com/) version of [privnote](https://privnote.com)

## Goals

* Have a web service that allows a user to:
        * create private notes with limited uses
        * fetch notes via url

## Non Goals

* deployment on actual prod environment

## Requirements

* Use Rocket as a web framework
* MVP:
        * POST /notes to create notes
        * GET /notes/<hash> to fetch (and delete) notes
* Learn some rust
* Have some fun

## TODOS

[] Encrypt notes on creation
[] Password protect notes
[] Add auto-expire to notes
[] Add usage counter to notes
[] Add unique urls to notes
