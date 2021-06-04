//
// Created by Logan Keenan on 6/4/21.
//

import UIKit

class NavigationController: UINavigationController {

    var uri: String = ""

    public convenience init(uri: String) {
        self.init()
        self.uri = uri
    }

    private func handle_request_response(request: AppRequest) {
        var response = AppService().make_request(appRequest: request)
        var wasRedirected = false

        if response.status_code == 302 {
            wasRedirected = true
            let location: String? = response.headers?["Location"]
            let request = AppRequest(uri: "\(location!)", method: "GET")
            response = AppService().make_request(appRequest: request)
        }
        let new_uri: String = response.headers!["Content-Location"]!

        if wasRedirected && getPreviousController().uri == new_uri {
            popViewController(animated: true)
            getCurrentController().reload()
        } else if wasRedirected {
            let controller = ViewController(uri: new_uri)
            controller.delegate = self
            popViewController(animated: false)
            pushViewController(controller, animated: true)
        } else {
            let controller = ViewController(uri: new_uri)
            controller.delegate = self
            pushViewController(controller, animated: true)
        }
    }

    private func getCurrentController() -> ViewController {
        viewControllers[viewControllers.count - 1] as! ViewController
    }

    private func getPreviousController() -> ViewController {
        if viewControllers.count == 1 {
            return viewControllers[0] as! ViewController
        }
        return viewControllers[viewControllers.count - 2] as! ViewController
    }

    override func viewDidLoad() {
        let request = AppRequest(uri: self.uri, method: "GET")
        let attributes = [NSAttributedString.Key.font: UIFont(name: "Nunito-Bold", size: 18)!]
        navigationBar.titleTextAttributes = attributes
        handle_request_response(request: request)
    }
}

extension NavigationController: ViewControllerDelegate {
    func makeAppRequest(_ viewController: ViewController, request_as_json: String) {
        let request: AppRequest? = try? JSONDecoder().decode(AppRequest.self, from: request_as_json.data(using: .utf8)!)

        handle_request_response(request: request!)
    }
}
