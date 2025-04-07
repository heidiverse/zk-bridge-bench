plugins {
	// Kotlin & KMP plugins
	alias(libs.plugins.kotlin.multiplatform) apply false
	alias(libs.plugins.kotlin.atomicfu) apply false

	// Android specific plugins
	alias(libs.plugins.android.library) apply false
	alias(libs.plugins.android.application) apply false
	alias(libs.plugins.kotlin.android) apply false
	alias(libs.plugins.compose.compiler) apply false

	// Rust plugins
	alias(libs.plugins.trixnity.uniffi) apply false
	alias(libs.plugins.trixnity.cargo) apply false

}
