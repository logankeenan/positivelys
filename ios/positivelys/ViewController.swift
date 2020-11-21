//
//  ViewController.swift
//  positivelys
//
//  Created by Logan Keenan on 10/30/20.
//
//

import UIKit
import Foundation
import WebKit

protocol ViewControllerDelegate: class {
    func makeAppRequest(_ viewController: ViewController, url: String, method: String, body: String)
}

class ViewController: UIViewController, WKUIDelegate, WKScriptMessageHandler {

    weak var delegate: ViewControllerDelegate?
    var webView: WKWebView!

    private var html_markup: String!



    public convenience init(html_markup: String) {
        self.init()
        self.html_markup = html_markup
    }

    override func loadView() {
        let webConfiguration = WKWebViewConfiguration()
        webConfiguration.userContentController.add(self, name: "makeAppRequest")
        webView = WKWebView(frame: .zero, configuration: webConfiguration)
        webView.uiDelegate = self
        view = webView
    }

    override func viewDidLoad() {
        super.viewDidLoad()
        // Do any additional setup after loading the view.

//        let database_path : String = NSSearchPathForDirectoriesInDomains(.documentDirectory, .userDomainMask, true)[0] + "/database.sqlite";
//
//        let request = "{\"body\":null,\"method\":\"GET\",\"path\":\"/positivelys\"}"
//        let appContext = "{\"database_path\":\"" + database_path + "\"}"
//        let result = make_app_request(request, appContext)
//        let query_result = String(cString: result!)
//
//        make_app_request_free(UnsafeMutablePointer(mutating: result))
//
//        let response: AppResponse? = try? JSONDecoder().decode(AppResponse.self, from: query_result.data(using: .utf8)!)

        webView.loadHTMLString(self.html_markup, baseURL: URL(string: "https://logankeenan.com"))
    }

    func userContentController(_ userContentController: WKUserContentController, didReceive message: WKScriptMessage) {
        if message.name == "makeAppRequest" {
            guard let params = message.body as? [String: String],
                  let url = params["url"],
                  let method = params["method"],
                  let body = params["body"] else {
                return
            }

            self.delegate?.makeAppRequest(self, url: url, method: method, body: body)
        }
    }
}
