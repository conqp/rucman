pub enum Question {
    InstalledIgnoredPackage,
    ReplacePackage,
    RemoveConflictingPackage,
    RemoveCorruptedPackage,
    RemoveUnresolvableTargets,
    SelectProvider,
    ImportPGPKey,
}
