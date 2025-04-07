import SwiftUI
import bench_lib

struct ContentView: View {
    let greet = bench_lib.Bench.companion.getHello()

	var body: some View {
		Text(greet)
	}
}

struct ContentView_Previews: PreviewProvider {
	static var previews: some View {
		ContentView()
	}
}
