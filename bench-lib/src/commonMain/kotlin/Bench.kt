import uniffi.signatures_bench.Preparation
import kotlin.time.measureTime

class Bench {
    private val preparation: Preparation = uniffi.signatures_bench.prepare()
    private var vc: String? = null
    private var vp: String? = null

    private fun issue(): String = uniffi.signatures_bench.zkpIssue(
        issuer = preparation.issuer,
        deviceBinding = preparation.deviceBinding
    )

    private fun present(vc: String): String =
        uniffi.signatures_bench.zkpPresent(
            vc,
            issuerPk = preparation.issuer.publicKey,
            provingKeys = preparation.provingKeys,
            publicKey = preparation.dbPublicKey,
            message = preparation.message,
            messageSignature = preparation.messageSignature
        )

    fun benchIssue(n: Int): Long {
        return measureTime {
            for (x in 0..<n) {
                vc = issue()
            }
        }.inWholeMilliseconds / n
    }

    fun benchPresent(n: Int): Long {
        val vc = this.vc ?: issue()

        return measureTime {
            for (x in 0..<n) {
                vp = present(vc)
            }
        }.inWholeMilliseconds / n
    }

    fun benchVerify(n: Int): Long {
        val vc = this.vc ?: issue()
        val vp = this.vp ?: present(vc)

        return measureTime {
            for (x in 0..<n) {
                uniffi.signatures_bench.zkpVerify(
                    presentation = vp,
                    issuerPk = preparation.issuer.publicKey,
                    verifyingKeys = preparation.verifyingKeys,
                    message = preparation.message
                )
            }
        }.inWholeMilliseconds / n
    }

    companion object {
        fun getHello(): String = uniffi.signatures_bench.hello()
    }
}