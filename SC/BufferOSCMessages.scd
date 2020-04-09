~bufServer = NetAddr("127.0.0.1", 1234); //the scnbufffer address

//b_alloc: bufnum, numFrames, numChans, oscreplymsg
~bufServer.sendMsg("/b_alloc", 1, 44100, 2, "/b_reply msg in bytes");

//b_allocRead: bufnum, pathname, startFrame, numFrames, oscreplymsg
~bufServer.sendMsg("/b_allocRead", 2, "~/funfile.wav", 0, 44100, "/my reply")

//b_allocReadChannel: bufnum, pathname, startFram, numFrames, chanIndexes..., oscreplymsg
~bufServer.sendMsg("/b_allocReadChannel", 3, "goodfile.wav", 0, 44100, 0,2,4,8, "/myreply")

//b_read: bufnum, pathname, startFrame, numFrames, bufStartFrame, bufNumFrames, leaveOpen, oscreplymsg
~bufServer.sendMsg("/b_read", 4, 0, 44100, 0, 44100, true, "/myreply");

//b_readChannel: bufnum, pathname, startFrame, numFrames, bufStartFrame, bufNumFrames, leaveOpen, channelIndexes... oscreplymsg
~bufServer.sendMsg("/b_readChannel", 5, "funsound.wav", 0, 44100, 0, 44100, false, 1,3,5,7, "/myreply");

//b_write: bufnum, pathname, headerFormat, sampleFormat, numFrames, bufStartFrame, leaveOpen, oscreplymsg
~bufServer.sendMsg("/b_write", 1, "recordedSound.aiff", "aiff", "int16", 44100, 0, false, "/myreply");

//b_free: bufnum, oscreplymsg
~bufServer.sendMsg("/b_free", 2, "/myrepl");

//b_zero: bufnum, oscreplymsg
~bufServer.sendMsg("/b_zero", 3, "/myreplt");

//b_set: bufnum, i0, v0, i1, v1 ... iN, vN
~bufServer.sendMsg("/b_read", 4321);

//b_setn: bufnum, i0, n0, v0, i1, n1, v1 ... iN, nN, vN
~bufServer.sendMsg("/b_read", 4321);

//b_fill: bufnum, numFrames, value
~bufServer.sendMsg("/b_fill", 5, 1000, 0.5);

//b_gen: bufnum, cmdName, cmdArgs
~bufServer.sendMsg("/b_gen", 5, 1000, 0.5);

//b_close: bufnum, oscreplymsg
~bufServer.sendMsg("/b_close", 3, "/myreply");

//b_query: bufnum
~bufServer.sendMsg("/b_query", 3);

//b_get: bufnum, frameIndex
~bufServer.sendMsg("/b_get", 3, 33);

//b_getn: bufnum, startFrame, numFrames
~bufServer.sendMsg("/b_getn", 3, 33, 1000);

//Uknown OSC opath
~bufServer.sendMsg("/b_nothing", 3, 33, 1000);
