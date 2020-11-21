//
// Created by Logan Keenan on 11/16/20.
//

import UIKit

class ApplicationController: UINavigationController {
    override func viewDidLoad() {
        super.viewDidLoad()
        let database_path : String = NSSearchPathForDirectoriesInDomains(.documentDirectory, .userDomainMask, true)[0] + "/database.sqlite";

        let request = "{\"body\":null,\"method\":\"GET\",\"url\":\"/positivelys\"}"
        let appContext = "{\"database_path\":\"" + database_path + "\"}"
        let result = make_app_request(request, appContext)
        let query_result = String(cString: result!)

        make_app_request_free(UnsafeMutablePointer(mutating: result))

        let response: AppResponse? = try? JSONDecoder().decode(AppResponse.self, from: query_result.data(using: .utf8)!)

        let controller = ViewController(html_markup: (response?.body)!)
        controller.delegate = self

        pushViewController(controller, animated: true)
    }
}

extension ApplicationController: ViewControllerDelegate {
    func makeAppRequest(_ viewController: ViewController, url: String, method: String, body: String) {
        let database_path : String = NSSearchPathForDirectoriesInDomains(.documentDirectory, .userDomainMask, true)[0] + "/database.sqlite"

        let request = "{\"body\":\"\(body)\",\"method\":\"\(method)\",\"url\":\"\(url.replacingOccurrences(of: "https://logankeenan.com", with: ""))\"}"

        let appContext = "{\"database_path\":\"" + database_path + "\"}"
        let result = make_app_request(request, appContext)
        let query_result = String(cString: result!)

        make_app_request_free(UnsafeMutablePointer(mutating: result))

        let response: AppResponse? = try? JSONDecoder().decode(AppResponse.self, from: query_result.data(using: .utf8)!)

        let controller = ViewController(html_markup: (response?.body)!)
        controller.delegate = self

        pushViewController(controller, animated: true)
    }


}