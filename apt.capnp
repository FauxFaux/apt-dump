@0xaf696212bdf0eef6;

# code/apt/apt-pkg/tagfile-keys.list

struct Item {
    union {
        end       @0 :Void;
        rawSource @1 :RawSource;
        rawBinary @2 :RawBinary;
        source    @3 :Source;
    }
}

struct RawSource {
    package  @0 :Text;
    version  @1 :Text;
    index    @2 :Text;

    binaries @3 :List(Text);
    files    @4 :List(File);

    entries  @5 :List(Entry);
}

struct RawBinary {
    index   @0 :IndexFile;
    entries @1 :List(Entry);
}

struct File {
    name   @0 :Text;
    size   @1 :UInt64;
    md5    @2 :Text;
    sha1   @3 :Text;
    sha256 @4 :Text;
    sha512 @5 :Text;
}

struct Entry {
    key   @0 :Text;
    value @1 :Text;
}

struct Source {
    package     @0 :Text;
    version     @1 :Text;
    index       @2 :Text;

    priority    @3 :Priority;
    arch        @4 :List(Text);

    format :union {
        unknown     @5 :Void;
        original    @6 :Void;
        quilt3dot0  @7 :Void;
        native3dot0 @8 :Void;
        git3dot0    @9 :Void;
    }

    binaries    @10 :List(SourceBinary);
    files       @11 :List(File);
    vcs         @12 :List(Vcs);

    buildDep           @13 :List(Dependency);
    buildDepArch       @14 :List(Dependency);
    buildDepIndep      @15 :List(Dependency);
    buildConflict      @16 :List(Dependency);
    buildConflictArch  @17 :List(Dependency);
    buildConflictIndep @18 :List(Dependency);

    unparsed @19 :UnparsedSource;
}

struct Dependency {
    alternate @0 :List(SingleDependency);
}

struct SingleDependency {
    package            @0 :Text;
    arch               @1 :Text;
    versionConstraints @2 :List(Constraint);
    archFilter         @3 :List(Text);
    stageFilter        @4 :List(Text);
}

struct Constraint {
    version @0 :Text;
    operator :union {
        ge @1 :Void;
        eq @2 :Void;
        le @3 :Void;
        gt @4 :Void;
        lt @5 :Void;
    }
}

struct Vcs {
    description @0 :Text;

    type :union {
        browser @1 :Void;
        arch    @2 :Void;
        bzr     @3 :Void;
        cvs     @4 :Void;
        darcs   @5 :Void;
        git     @6 :Void;
        hg      @7 :Void;
        mtn     @8 :Void;
        svn     @9 :Void;
    }

    tag :union {
        vcs      @10 :Void;
        orig     @11 :Void;
        debian   @12 :Void;
        upstream @13 :Void;
    }
}

struct SourceBinary {
    name      @0 :Text;
    style     @1 :Text;
    section   @2 :Text;

    priority  @3 :Priority;
    extras    @4 :List(Text);
}

# https://www.debian.org/doc/debian-policy/#priorities
struct Priority {
    union {
        unknown   @0 :Void;
        required  @1 :Void;
        important @2 :Void;
        standard  @3 :Void;
        optional  @4 :Void;
        extra     @5 :Void;
        source    @6 :Void;
    }
}

struct IndexFile {
    archive   @0 :Text;
    version   @1 :Text;
    origin    @2 :Text;
    codename  @3 :Text;
    label     @4 :Text;
    site      @5 :Text;
    component @6 :Text;
    arch      @7 :Text;
    type      @8 :Text;
}

## generated by gen.py

struct UnparsedSource {
    autobuild              @0 :Text;
    breaks                 @1 :Text;
    bugs                   @2 :Text;
    buildIndepArchitecture @3 :Text;
    builtForProfiles       @4 :Text;
    builtUsing             @5 :Text;
    class                  @6 :Text;
    comment                @7 :Text;
    conffiles              @8 :Text;
    configVersion          @9 :Text;
    conflicts              @10 :Text;
    depends                @11 :Text;
    description            @12 :Text;
    descriptionMd5         @13 :Text;
    dgit                   @14 :Text;
    directory              @15 :Text;
    dmUploadAllowed        @16 :Text;
    enhances               @17 :Text;
    essential              @18 :Text;
    extraSourceOnly        @19 :Text;
    filename               @20 :Text;
    goImportPath           @21 :Text;
    homepage               @22 :Text;
    important              @23 :Text;
    installedSize          @24 :Text;
    installerMenuItem      @25 :Text;
    kernelVersion          @26 :Text;
    maintainer             @27 :Text;
    md5sum                 @28 :Text;
    msdosFilename          @29 :Text;
    multiArch              @30 :Text;
    optional               @31 :Text;
    origin                 @32 :Text;
    originalMaintainer     @33 :Text;
    packageRevision        @34 :Text;
    packageType            @35 :Text;
    preDepends             @36 :Text;
    provides               @37 :Text;
    python3Version         @38 :Text;
    pythonVersion          @39 :Text;
    recommended            @40 :Text;
    recommends             @41 :Text;
    replaces               @42 :Text;
    revision               @43 :Text;
    rubyVersions           @44 :Text;
    section                @45 :Text;
    sha1                   @46 :Text;
    sha256                 @47 :Text;
    sha512                 @48 :Text;
    size                   @49 :Text;
    standardsVersion       @50 :Text;
    status                 @51 :Text;
    subarchitecture        @52 :Text;
    suggests               @53 :Text;
    tag                    @54 :Text;
    task                   @55 :Text;
    testsuite              @56 :Text;
    testsuiteRestrictions  @57 :Text;
    testsuiteTriggers      @58 :Text;
    triggersAwaited        @59 :Text;
    triggersPending        @60 :Text;
    uploaders              @61 :Text;
}
