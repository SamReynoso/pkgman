pub enum AppEvent {
	Tick,
	Key(crossterm::event::KeyEvent),
	DbLoaded(Vec<crate::app::Package>),
	AurLoaded(Vec<crate::app::Package>),
	Message(String, u64, bool), // msg, secs, keep
	ConsoleChunk(Vec<u8>),      // raw pty output, fed to the vt100 screen
	ConsoleFinished(bool),
	PtyStarted(crate::app::ConsolePty), // console subprocess spawned, handles for input/resize
	LoadingDone, // clear the loading spinner without altering the status message
	Resize,
	AurDetailsLoaded(Box<crate::app::Package>),
	AurDetailsFailed(String), // reset the fetching marker so the row isn't stuck
	DepTreeLoaded(String, Result<Vec<String>, String>),
	WikiLoaded(String, Result<Vec<String>, String>),
}
