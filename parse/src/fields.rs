// GENERATED by gen.py; do not edit
#![cfg_attr(rustfmt, rustfmt_skip)]

use apt_capnp::unparsed_source;
use apt_capnp::unparsed_binary;
use errors::*;
use blank_to_null;

pub const HANDLED_FIELDS_SOURCE: [&'static str; 80] = [
    "Architecture",
    "Binary",
    "Build-Conflicts",
    "Build-Conflicts-Arch",
    "Build-Conflicts-Indep",
    "Build-Depends",
    "Build-Depends-Arch",
    "Build-Depends-Indep",
    "Checksums-Md5",
    "Checksums-Sha1",
    "Checksums-Sha256",
    "Checksums-Sha512",
    "Debian-Vcs-Arch",
    "Debian-Vcs-Browse",
    "Debian-Vcs-Browser",
    "Debian-Vcs-Bzr",
    "Debian-Vcs-Cvs",
    "Debian-Vcs-Darcs",
    "Debian-Vcs-Git",
    "Debian-Vcs-Hg",
    "Debian-Vcs-Mtn",
    "Debian-Vcs-Svn",
    "Files",
    "Format",
    "Orig-Maintainer",
    "Orig-Vcs-Arch",
    "Orig-Vcs-Browse",
    "Orig-Vcs-Browser",
    "Orig-Vcs-Bzr",
    "Orig-Vcs-Cvs",
    "Orig-Vcs-Darcs",
    "Orig-Vcs-Git",
    "Orig-Vcs-Hg",
    "Orig-Vcs-Mtn",
    "Orig-Vcs-Svn",
    "Original-Vcs-Arch",
    "Original-Vcs-Browse",
    "Original-Vcs-Browser",
    "Original-Vcs-Bzr",
    "Original-Vcs-Cvs",
    "Original-Vcs-Darcs",
    "Original-Vcs-Git",
    "Original-Vcs-Hg",
    "Original-Vcs-Mtn",
    "Original-Vcs-Svn",
    "Package",
    "Package-List",
    "Priority",
    "Source",
    "Upstream-Vcs-Arch",
    "Upstream-Vcs-Browse",
    "Upstream-Vcs-Browser",
    "Upstream-Vcs-Bzr",
    "Upstream-Vcs-Cvs",
    "Upstream-Vcs-Darcs",
    "Upstream-Vcs-Git",
    "Upstream-Vcs-Hg",
    "Upstream-Vcs-Mtn",
    "Upstream-Vcs-Svn",
    "Vcs-Arch",
    "Vcs-Browse",
    "Vcs-Browser",
    "Vcs-Bzr",
    "Vcs-Cvs",
    "Vcs-Darcs",
    "Vcs-Git",
    "Vcs-Hg",
    "Vcs-Mtn",
    "Vcs-Svn",
    "Vcs-Upstream-Arch",
    "Vcs-Upstream-Browse",
    "Vcs-Upstream-Browser",
    "Vcs-Upstream-Bzr",
    "Vcs-Upstream-Cvs",
    "Vcs-Upstream-Darcs",
    "Vcs-Upstream-Git",
    "Vcs-Upstream-Hg",
    "Vcs-Upstream-Mtn",
    "Vcs-Upstream-Svn",
    "Version",
];

pub fn set_field_source(key: &str, val: &str, builder: &mut unparsed_source::Builder) -> Result<()> {
    match key {
        "Autobuild" => blank_to_null(val, |x| builder.set_autobuild(x)),
        "Build-Indep-Architecture" => blank_to_null(val, |x| builder.set_build_indep_architecture(x)),
        "Comment" => blank_to_null(val, |x| builder.set_comment(x)),
        "Dgit" => blank_to_null(val, |x| builder.set_dgit(x)),
        "Directory" => blank_to_null(val, |x| builder.set_directory(x)),
        "Dm-Upload-Allowed" => blank_to_null(val, |x| builder.set_dm_upload_allowed(x)),
        "Extra-Source-Only" => blank_to_null(val, |x| builder.set_extra_source_only(x)),
        "Go-Import-Path" => blank_to_null(val, |x| builder.set_go_import_path(x)),
        "Homepage" => blank_to_null(val, |x| builder.set_homepage(x)),
        "Maintainer" => blank_to_null(val, |x| builder.set_maintainer(x)),
        "Origin" => blank_to_null(val, |x| builder.set_origin(x)),
        "Original-Maintainer" => blank_to_null(val, |x| builder.set_original_maintainer(x)),
        "Python-Version" => blank_to_null(val, |x| builder.set_python_version(x)),
        "Python3-Version" => blank_to_null(val, |x| builder.set_python3_version(x)),
        "Ruby-Versions" => blank_to_null(val, |x| builder.set_ruby_versions(x)),
        "Section" => blank_to_null(val, |x| builder.set_section(x)),
        "Standards-Version" => blank_to_null(val, |x| builder.set_standards_version(x)),
        "Testsuite" => blank_to_null(val, |x| builder.set_testsuite(x)),
        "Testsuite-Restrictions" => blank_to_null(val, |x| builder.set_testsuite_restrictions(x)),
        "Testsuite-Triggers" => blank_to_null(val, |x| builder.set_testsuite_triggers(x)),
        "Uploaders" => blank_to_null(val, |x| builder.set_uploaders(x)),

        // Typos
        "Orig-Maintainer" => blank_to_null(val, |x| builder.set_original_maintainer(x)),

        other => bail!("unrecognised source field: {}", other),
    }

    Ok(())
}

pub const HANDLED_FIELDS_BINARY: [&'static str; 2] = [
    "Package",
    "Version",
];

pub fn set_field_binary(key: &str, val: &str, builder: &mut unparsed_binary::Builder) -> Result<()> {
    match key {
        "Auto-Built-Package" => blank_to_null(val, |x| builder.set_auto_built_package(x)),
        "Breaks" => blank_to_null(val, |x| builder.set_breaks(x)),
        "Bugs" => blank_to_null(val, |x| builder.set_bugs(x)),
        "Build-Essential" => blank_to_null(val, |x| builder.set_build_essential(x)),
        "Build-Ids" => blank_to_null(val, |x| builder.set_build_ids(x)),
        "Built-For-Profiles" => blank_to_null(val, |x| builder.set_built_for_profiles(x)),
        "Built-Using" => blank_to_null(val, |x| builder.set_built_using(x)),
        "Class" => blank_to_null(val, |x| builder.set_class(x)),
        "Conffiles" => blank_to_null(val, |x| builder.set_conffiles(x)),
        "Config-Version" => blank_to_null(val, |x| builder.set_config_version(x)),
        "Conflicts" => blank_to_null(val, |x| builder.set_conflicts(x)),
        "Depends" => blank_to_null(val, |x| builder.set_depends(x)),
        "Description" => blank_to_null(val, |x| builder.set_description(x)),
        "Description-en" => blank_to_null(val, |x| builder.set_description_en(x)),
        "Description-en_GB" => blank_to_null(val, |x| builder.set_description_en_gb(x)),
        "Description-md5" => blank_to_null(val, |x| builder.set_description_md5(x)),
        "Enhances" => blank_to_null(val, |x| builder.set_enhances(x)),
        "Essential" => blank_to_null(val, |x| builder.set_essential(x)),
        "Filename" => blank_to_null(val, |x| builder.set_filename(x)),
        "Files" => blank_to_null(val, |x| builder.set_files(x)),
        "Ghc-Package" => blank_to_null(val, |x| builder.set_ghc_package(x)),
        "Go-Import-Path" => blank_to_null(val, |x| builder.set_go_import_path(x)),
        "Gstreamer-Decoders" => blank_to_null(val, |x| builder.set_gstreamer_decoders(x)),
        "Gstreamer-Elements" => blank_to_null(val, |x| builder.set_gstreamer_elements(x)),
        "Gstreamer-Encoders" => blank_to_null(val, |x| builder.set_gstreamer_encoders(x)),
        "Gstreamer-Uri-Sinks" => blank_to_null(val, |x| builder.set_gstreamer_uri_sinks(x)),
        "Gstreamer-Uri-Sources" => blank_to_null(val, |x| builder.set_gstreamer_uri_sources(x)),
        "Gstreamer-Version" => blank_to_null(val, |x| builder.set_gstreamer_version(x)),
        "Homepage" => blank_to_null(val, |x| builder.set_homepage(x)),
        "Important" => blank_to_null(val, |x| builder.set_important(x)),
        "Installed-Size" => blank_to_null(val, |x| builder.set_installed_size(x)),
        "Installer-Menu-Item" => blank_to_null(val, |x| builder.set_installer_menu_item(x)),
        "Kernel-Version" => blank_to_null(val, |x| builder.set_kernel_version(x)),
        "License" => blank_to_null(val, |x| builder.set_license(x)),
        "Lua-Versions" => blank_to_null(val, |x| builder.set_lua_versions(x)),
        "MD5sum" => blank_to_null(val, |x| builder.set_md5sum(x)),
        "MSDOS-Filename" => blank_to_null(val, |x| builder.set_msdos_filename(x)),
        "Maintainer" => blank_to_null(val, |x| builder.set_maintainer(x)),
        "Modaliases" => blank_to_null(val, |x| builder.set_modaliases(x)),
        "Multi-Arch" => blank_to_null(val, |x| builder.set_multi_arch(x)),
        "Npp-Applications" => blank_to_null(val, |x| builder.set_npp_applications(x)),
        "Npp-Description" => blank_to_null(val, |x| builder.set_npp_description(x)),
        "Npp-File" => blank_to_null(val, |x| builder.set_npp_file(x)),
        "Npp-Mimetype" => blank_to_null(val, |x| builder.set_npp_mimetype(x)),
        "Npp-Name" => blank_to_null(val, |x| builder.set_npp_name(x)),
        "Optional" => blank_to_null(val, |x| builder.set_optional(x)),
        "Origin" => blank_to_null(val, |x| builder.set_origin(x)),
        "Original-Maintainer" => blank_to_null(val, |x| builder.set_original_maintainer(x)),
        "Package-Revision" => blank_to_null(val, |x| builder.set_package_revision(x)),
        "Package-Type" => blank_to_null(val, |x| builder.set_package_type(x)),
        "Phased-Update-Percentage" => blank_to_null(val, |x| builder.set_phased_update_percentage(x)),
        "Postgresql-Catversion" => blank_to_null(val, |x| builder.set_postgresql_catversion(x)),
        "Postgresql-Version" => blank_to_null(val, |x| builder.set_postgresql_version(x)),
        "Pre-Depends" => blank_to_null(val, |x| builder.set_pre_depends(x)),
        "Provides" => blank_to_null(val, |x| builder.set_provides(x)),
        "Python-Egg-Name" => blank_to_null(val, |x| builder.set_python_egg_name(x)),
        "Python-Version" => blank_to_null(val, |x| builder.set_python_version(x)),
        "Python3-Version" => blank_to_null(val, |x| builder.set_python3_version(x)),
        "Recommended" => blank_to_null(val, |x| builder.set_recommended(x)),
        "Recommends" => blank_to_null(val, |x| builder.set_recommends(x)),
        "Replaces" => blank_to_null(val, |x| builder.set_replaces(x)),
        "Revision" => blank_to_null(val, |x| builder.set_revision(x)),
        "Ruby-Versions" => blank_to_null(val, |x| builder.set_ruby_versions(x)),
        "SHA1" => blank_to_null(val, |x| builder.set_sha1(x)),
        "SHA256" => blank_to_null(val, |x| builder.set_sha256(x)),
        "SHA512" => blank_to_null(val, |x| builder.set_sha512(x)),
        "Section" => blank_to_null(val, |x| builder.set_section(x)),
        "Size" => blank_to_null(val, |x| builder.set_size(x)),
        "Source" => blank_to_null(val, |x| builder.set_source(x)),
        "Status" => blank_to_null(val, |x| builder.set_status(x)),
        "Subarchitecture" => blank_to_null(val, |x| builder.set_subarchitecture(x)),
        "Suggests" => blank_to_null(val, |x| builder.set_suggests(x)),
        "Supported" => blank_to_null(val, |x| builder.set_supported(x)),
        "Tads2-Version" => blank_to_null(val, |x| builder.set_tads2_version(x)),
        "Tads3-Version" => blank_to_null(val, |x| builder.set_tads3_version(x)),
        "Tag" => blank_to_null(val, |x| builder.set_tag(x)),
        "Task" => blank_to_null(val, |x| builder.set_task(x)),
        "Triggers-Awaited" => blank_to_null(val, |x| builder.set_triggers_awaited(x)),
        "Triggers-Pending" => blank_to_null(val, |x| builder.set_triggers_pending(x)),
        "Vendor" => blank_to_null(val, |x| builder.set_vendor(x)),
        "Xul-Appid" => blank_to_null(val, |x| builder.set_xul_appid(x)),

        // Typos
        "Package_Revision" => blank_to_null(val, |x| builder.set_package_revision(x)),

        other => bail!("unrecognised binary field: {}", other),
    }

    Ok(())
}
