enum State {
    New,
    Running,
    Sleeping,
    Restart,
    Zombie,
}

struct User {}

struct Process {}

struct File {}

trait Server {
    fn get_name(&self) -> &String;
    fn boot(&mut self);
    fn kill(&mut self, restart: bool);
}

struct FileServer {
    name: String,
    state: State,
}

impl Server for FileServer {
    fn get_name(&self) -> &String {
        &self.name
    }
      
    fn boot(&mut self) {
        println!("booting the {}", self.name);
        self.state = State::Running;
    }

    fn kill(&mut self, restart: bool) {
        println!("killing {}", self.name);
        match restart {
            true => self.state = State::Restart,
            false => self.state = State::Zombie,
        }
    }
}

impl FileServer {
    fn new() -> FileServer {
        FileServer {
            name: "FileServer".to_string(),
            state: State::New,
        }
    }

    fn create_file(&self, user: String, name: String, permissions: String) {
        println!("trying to create the file '{}' for user '{}' with permissions {}",
            name,
            user,
            permissions
        )
    }
}

struct ProcessServer {
    name: String,
    state: State,
}

impl Server for ProcessServer {
    fn get_name(&self) -> &String {
        &self.name
    }
      
    fn boot(&mut self) {
        println!("booting the {}", self.name);
        self.state = State::Running;
    }

    fn kill(&mut self, restart: bool) {
        println!("killing {}", self.name);
        match restart {
            true => self.state = State::Restart,
            false => self.state = State::Zombie,
        }
    }
}

impl ProcessServer {
    fn new() -> ProcessServer {
        ProcessServer {
            name: "ProcessServer".to_string(),
            state: State::New,
        }
    }

    fn create_process(&self, user: String, name: String) {
        println!("trying to create the process '{}' for user '{}'",
            name,
            user
        )
    }
}

struct WindowServer {}

struct NetworkServer {}

struct OperatingSystem {
    fs: FileServer,
    ps: ProcessServer,
}

impl OperatingSystem {
    fn new() -> OperatingSystem {
        OperatingSystem {
            fs: FileServer::new(),
            ps: ProcessServer::new(),
        }
    }

    fn start(&mut self) {
        self.fs.boot();
        self.ps.boot();
    }

    fn create_file(&self, user: String, name: String, permissions: String) {
        self.fs.create_file(user, name, permissions);
    }

    fn create_process(&self, user: String, name: String) {
        self.ps.create_process(user, name);
    }
}

fn main() {
    let mut os = OperatingSystem::new();
    os.start();
    os.create_file(
        "foo".to_string(),
        "hello".to_string(),
        "-rw-r-r".to_string()
    );
    os.create_process(
        "bar".to_string(),
        "ls /tmp".to_string()
    );
}
