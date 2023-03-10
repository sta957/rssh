use std::net::TcpStream;
use std::io::Read;
use std::path::Path;
use std::process::exit;
use ssh2::Session;
use std::env;

const VERSION: &str = "0.1";
const USERNAME: &str = "testuser";
const PASSWORD: &str = "password";

pub struct SSHConnection {
    sess: Session,
    address: String,
    port: u32,
    user: String,
    pass: String,
}

impl SSHConnection {
    fn connect(&mut self) {
        let ip_add = format!("{}:{}", self.address, self.port);
        let tcp = TcpStream::connect(ip_add).unwrap();

        self.sess.set_tcp_stream(tcp);
        self.sess.handshake().unwrap();
    
        self.sess.userauth_password(&self.user, &self.pass).unwrap();
    }
    
    pub fn command(&self, cmd: &str) {
        let mut channel = self.sess.channel_session().unwrap();
        channel.exec(cmd).unwrap();
        let mut s = String::new();
        channel.read_to_string(&mut s).unwrap();
        println!("{}", s);
        channel.wait_close().unwrap();
    }
    
    pub fn upload(&self, _path: &str, _filename: &str, _content: &str) {
        todo!("implement the upload of files");
    }
    
    pub fn download(&self, path: &str) {
        let (_, filename) = path.rsplit_once('/').unwrap();
        let (mut remote_file, stat) = self.sess.scp_recv(Path::new(path)).unwrap();
        
        println!("remote file size: {}", stat.size());
        let mut contents = Vec::new();
        remote_file.read_to_end(&mut contents).unwrap();

        remote_file.send_eof().unwrap();
        remote_file.wait_eof().unwrap();
        remote_file.close().unwrap();
        remote_file.wait_close().unwrap();

        std::fs::write(filename, &contents).unwrap();
    }
}

fn help() {
    let prog: &str = "rssh";
    println!("Rust-based SSH-Manager\nVersion: {VERSION}\nusage:
    {prog} [-cdmsu] <command>
        -c <HOST>:<PORT> 
                Connect to a specific host (DEFAULT OPTION) 
                e.g. {prog} -c 127.0.0.1:22
                The default port is 22 if not defined
        -d <HOST>:<PORT> <PATH>
                Download a file from given host to pwd
                e.g {prog} -d 127.0.0.1 testfile.log
                Relative paths are possible
        -m
                Modify the locally stored hostlist and keypairs
                Hosts are stored in .config/rssh.hosts
        -s <HOST>:<PORT> <COMMAND>
                Send a specific command
                e.g. {prog} -s 'ls -la'
        -u <HOST>:<PORT>:<PATH> <FILE> 
                Upload a file to given host
                e.g. {prog} -u 127.0.0.1:/home/user/ test.log
                Port specification is not mandantory");
}

fn arg_parser(args: &Vec<String>) {
    //TODO: split args[2] by : and retrieve necessary data

    let mut ssh = SSHConnection{
        sess: Session::new().unwrap(),
        address: "127.0.0.1".to_owned(),
        port: 22,
        user: USERNAME.to_owned(),
        pass: PASSWORD.to_owned(),
    };

    match &*args[1] {
        "-c" => {
            //TODO: Implement interactive connection to host
            println!("Connect");
            ssh.connect();
            ssh.command("ls");
        },
        "-d" => {
            println!("Download");
            ssh.download(&*args[3]);
        },
        "-m" => {
            //TODO: Implement Modification of hostlist
            println!("Modify");
        },
        "-s" => {
            ssh.connect();
            ssh.command(&*args[3]);
        },
        "-u" => {
            //TODO: Implement Upload of file
            println!("Upload");
        },
        _ => {
            println!("Connect");
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => {
            help();
            exit(1);
        },
        2 | 3 => arg_parser(&args),
        _ => {
            help();
            exit(1);
        }
    }
}
