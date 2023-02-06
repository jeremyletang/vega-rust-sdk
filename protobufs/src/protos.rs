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

    pub mod data {
        pub mod v1 {
            include!(concat!(env!("OUT_DIR"), "/vega.data.v1.rs"));
        }
    }
}

pub mod datanode {
    pub mod api {
        pub mod v2 {
            include!(concat!(env!("OUT_DIR"), "/datanode.api.v2.rs"));
        }
    }
}

pub mod blockexplorer {
    pub mod api {
        pub mod v1 {
            include!(concat!(env!("OUT_DIR"), "/blockexplorer.api.v1.rs"));
        }
    }
}
