rootProject.name = "next-gen-signatures-bench"
enableFeaturePreview("TYPESAFE_PROJECT_ACCESSORS")

pluginManagement {
    fun RepositoryHandler.ubique() = maven {
        name = "ubique"
        val ubiqueMavenUrl = System.getenv("UB_ARTIFACTORY_URL_ANDROID")
            ?: System.getenv("ARTIFACTORY_URL_ANDROID")
            ?: extra["ubiqueMavenUrl"] as? String
            ?: ""
        val ubiqueMavenUser = System.getenv("UB_ARTIFACTORY_USER")
            ?: System.getenv("ARTIFACTORY_USER_NAME")
            ?: extra["ubiqueMavenUser"] as? String
            ?: ""
        val ubiqueMavenPass = System.getenv("UB_ARTIFACTORY_PASSWORD")
            ?: System.getenv("ARTIFACTORY_API_KEY")
            ?: extra["ubiqueMavenPass"] as? String
            ?: ""
        url = uri(ubiqueMavenUrl)
        credentials {
            username = ubiqueMavenUser
            password = ubiqueMavenPass
        }
        authentication {
            create<BasicAuthentication>("basic")
            create<DigestAuthentication>("digest")
        }
        content {
            includeGroupAndSubgroups("ch.ubique")
            includeGroupAndSubgroups("io.gitlab.trixnity")
        }
    }

    repositories {
        ubique()
        google {
            mavenContent {
                includeGroupAndSubgroups("androidx")
                includeGroupAndSubgroups("com.android")
                includeGroupAndSubgroups("com.google")
            }
        }
        gradlePluginPortal()
        mavenCentral()
    }

    dependencyResolutionManagement {
        repositories {
            ubique()
            google()
            mavenCentral()
        }
    }
}

include(":android-bench")
include(":bench-lib")