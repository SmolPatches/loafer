-- put this in ~/.config/mpv/scripts 
-- make all mpv sockets listen to commands from this socket
mp.set_property('input-ipc-server', '/tmp/loafer.sock') 
