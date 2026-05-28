import ch.ubique.uniffi.plugin.extensions.useRustUpLinker

plugins {
	alias(libs.plugins.kotlin.multiplatform)
	alias(libs.plugins.kotlin.atomicfu)
	alias(libs.plugins.android.library)
	alias(libs.plugins.kotlin.serialization)
	alias(libs.plugins.uniffi.plugin)
}

kotlin {
	compilerOptions {
		freeCompilerArgs.add("-Xexpect-actual-classes")
		freeCompilerArgs.add("-Xwhen-guards")
	}

	jvmToolchain(17)

	androidTarget {
		publishLibraryVariants = listOf("release")

	}
	jvm()
	listOf(
		iosX64(),
		iosArm64()
	).forEach { iosTarget ->
		iosTarget.binaries.framework {
			baseName = "bench-lib"
			isStatic = true
		}

		iosTarget.compilations.getByName("main") {
			useRustUpLinker()
		}
	}

	sourceSets {
		commonMain.dependencies {
			implementation(libs.uniffi.runtime)
			implementation(libs.kotlin.serialization)
		}

		commonTest.dependencies {
			implementation(libs.kotlin.test)
		}
	}
}

android {
	namespace = "ch.ubique.heidi.signatures.bench"
	compileSdk = libs.versions.android.compileSdk.get().toInt()

	defaultConfig {
		minSdk = libs.versions.android.minSdk.get().toInt()
		ndkVersion = "29.0.13599879"
		ndk {
			abiFilters += listOf("arm64-v8a")
		}
	}

	compileOptions {

		sourceCompatibility = JavaVersion.VERSION_17
		targetCompatibility = JavaVersion.VERSION_17
	}

}

uniffi {
	bindgenFromGitTag(
		"https://github.com/UbiqueInnovation/uniffi-kotlin-multiplatform-bindings.git",
		libs.versions.uniffi.bindgen.get()
	)
	generateFromLibrary()
}

cargo {
	packageDirectory = layout.projectDirectory.dir("rust")
//	debug.profile = CargoProfile.Release
}
