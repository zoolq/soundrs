# soundrs
A command line sound playback application written in rust

## Behind the scenes
On load the application initiates a couple of static, lazily loaded Mutext containers for the data pointers it interacts with

```
LIBRARY: HashMap<SongName, SongPointer>
PLAYLISTS: HashMap<PlaylistName, PlaylistPointer>
```

the LIBRARY contains pointers to all songs found in the specified song library directory and subdirectories
the PLAYLISTS contains pointers to all playlists in the specified playlist library directory and subdirectories

each playlist points to multiple songs which each point to a audio file

```
stuct Playlist {
    name,
    Vec<Songs>,
}

struct Song {
    name,
    SongPointer,
}
```