use crate::git::Git;
use crate::paket::Paket;
use crate::pkg::validator::PkgValidator;
use crate::result::Result;

pub struct Actions<'a> {
    pk: &'a Paket,
    git: Git,
}

impl<'a> Actions<'a> {
    pub fn new(pk: &'a Paket) -> Result<Self> {
        let git = Git::new(pk.paket_dir.clone())?;
        Ok(Self { pk, git })
    }

    /// Command action to install a new package
    pub fn install(self, pkg_name: &str) -> Result {
        let pkgv = PkgValidator::new(&pkg_name)?;
        let pkg_name = &pkgv.get_user_pkg_name();
        let pkg_tag = Some(pkgv.pkg_tag.as_ref());

        if self.pk.pkg_exists(pkg_name) {
            bail!(
                "package `{}` was already installed. Try to use the `up` command to upgrade it.",
                pkg_name
            );
        } else {
            self.git.clone(pkg_name, pkg_tag)?;

            // TODO: symlinking process of Fish shell package structure
        }

        Ok(())
    }

    /// Command action to update an existing package
    pub fn update(&mut self, pkg_name: &str) -> Result {
        let pkgv = PkgValidator::new(&pkg_name)?;
        let pkg_name = &pkgv.get_user_pkg_name();
        let pkg_tag = Some(pkgv.pkg_tag.as_ref());

        if !self.pk.pkg_exists(pkg_name) {
            bail!(
                "package `{}` was not installed. Try to use the `add` command to install it first.",
                pkg_name
            );
        }

        self.git.fetch(pkg_name, pkg_tag)?;
        self.git.checkout(pkg_name, Some("FETCH_HEAD"))?;

        Ok(())
    }

    /// Command action to remove an existing package
    pub fn remove(&self, pkg_name: &str) -> Result {
        println!("Remove: pkg {:?}", pkg_name);

        Ok(())
    }
}
