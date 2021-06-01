use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Opt {
    #[structopt(
        short,
        long = "url",
        help = "Domain of the asset registry",
        default_value = "https://asset-registry.tokenizedassetsdemo.iota.cafe"
    )]
    pub assets_domain: String,
    #[structopt(
        short = "i",
        long = "id",
        help = "Network ID amongst \"nectar\", \"pollen\", \"test\" and \"internal\"",
        default_value = "nectar"
    )]
    pub network_id: String,
    #[structopt(
        short = "t",
        long,
        help = "Delay in milliseconds between searches",
        default_value = "1000"
    )]
    delay: u64,
    #[structopt(short, long, help = "Username for asset registry")]
    user: String,
    #[structopt(short, long, help = "Password for asset registry")]
    password: String,
}

impl Opt {
    pub fn new() -> Self {
        Opt::from_args()
    }

    pub fn asset_domain(&self) -> &str {
        self.assets_domain.as_ref()
    }

    pub fn network_id(&self) -> &str {
        self.network_id.as_ref()
    }

    pub fn delay(&self) -> u64 {
        self.delay
    }

    pub fn user(&self) -> &str {
        self.user.as_ref()
    }

    pub fn password(&self) -> &str {
        self.password.as_ref()
    }
}
