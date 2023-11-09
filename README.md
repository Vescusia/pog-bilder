# Pog Bilder
Die offizielle Pog-Bilder Applikation für die Jungs.

## Concept
Pog Bilder is a project that tries to create a FOSS, self-hostable, feature rich, high performance and multiplatform chatting application.

The motivation for Pog Bilder is actually a frustration with most alternatives. 
Very lossy image compression and questionable security as well as needlessly heavy frontends are just some problems.
So, we decided to write it ourselfs. Can't be that hard, right?

The main ideals are:
-	simple: only implement what we need.
-	performant: bad performance leads to a bad experience.
-	capable: no (major) compromises in functionality and user experience.
-	FOSS: no explanation needed.

## Development

This codbase is split into two parts:
-	The Server, implemented in Rust
-	The Ui/Client, implemented in Flutter

#### Todo-List: (please update as necessary)
- Server
	- [x] Message Exchange
	- [x] persistent Message storing and request handling
	- [ ] Big-File Message support
	- [ ] feature-rich CLI + easy usage
	- [ ] Maximum Performance™

- Client
	- UI
		- [x] chat
		- [x] settings
		- File extension handling
			- [ ] Storage
			- [ ] Previews
	- Backend
		- [ ] Message exchange
		- [ ] persistent Message storing 
		- [ ] Big-File Message support

- CLI-Client
	- [x] planned

## Usage
### Server
> clone the repository
> `git clone https://github.com/Vescusia/pog-bilder.git`
>  move into directory
> `cd pog-bilder/server`
>  run server on port 6969
>  `cargo run -r -- -p 6969`
>  
>  (forward port + open firewall if necessary)

For more infos, see
`cargo run -r -- --help`

### Ui
...
