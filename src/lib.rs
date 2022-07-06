pub mod vega {

    include!(concat!(env!("OUT_DIR"), "/vega.rs"));

    pub mod commands {
        pub mod v1 {
            include!(concat!(env!("OUT_DIR"), "/vega.commands.v1.rs"));
        }
    }
    pub mod events {
        pub mod v1 {
            include!(concat!(env!("OUT_DIR"), "/vega.events.v1.rs"));
        }
    }
    pub mod wallet {
        pub mod v1 {
            include!(concat!(env!("OUT_DIR"), "/vega.wallet.v1.rs"));
        }
    }
    pub mod snapshot {
        pub mod v1 {
            include!(concat!(env!("OUT_DIR"), "/vega.snapshot.v1.rs"));
        }
    }
    pub mod checkpoint {
        pub mod v1 {
            include!(concat!(env!("OUT_DIR"), "/vega.checkpoint.v1.rs"));
        }
    }
    pub mod api {
        pub mod v1 {
            include!(concat!(env!("OUT_DIR"), "/vega.api.v1.rs"));
        }
    }
}

pub mod datanode {
    pub mod api {
        pub mod v1 {
            include!(concat!(env!("OUT_DIR"), "/datanode.api.v1.rs"));
        }
        pub mod v2 {
            include!(concat!(env!("OUT_DIR"), "/datanode.api.v1.rs"));
        }
    }
}

pub mod oracles {
    pub mod v1 {
        include!(concat!(env!("OUT_DIR"), "/oracles.v1.rs"));
    }
}
