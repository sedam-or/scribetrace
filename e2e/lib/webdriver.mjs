/**
 * ScribeTrace Lightweight W3C WebDriver Client
 *
 * Implements standard W3C WebDriver HTTP REST protocol for Tauri applications
 * driven via tauri-driver (port 4444) on Linux (WebKitWebDriver).
 * Pure ES module with 0 external npm dependencies (uses native fetch).
 */

export class WebDriverClient {
  constructor(baseUrl = 'http://127.0.0.1:4444') {
    this.baseUrl = baseUrl.replace(/\/+$/, '');
    this.sessionId = null;
  }

  async _request(method, endpoint, body = null) {
    const url = `${this.baseUrl}${endpoint}`;
    const options = {
      method,
      headers: {
        'Content-Type': 'application/json; charset=utf-8',
        'Accept': 'application/json'
      }
    };
    if (body !== null) {
      options.body = JSON.stringify(body);
    }

    const res = await fetch(url, options);
    let data;
    try {
      data = await res.json();
    } catch (err) {
      throw new Error(`WebDriver ${method} ${url} returned invalid JSON (HTTP ${res.status}): ${err.message}`);
    }

    if (!res.ok || (data && data.value && data.value.error)) {
      const errMsg = data?.value?.message || data?.value?.error || res.statusText;
      throw new Error(`WebDriver error (${res.status}) on ${method} ${endpoint}: ${errMsg}`);
    }

    return data.value;
  }

  /**
   * Initializes a new session with tauri-driver.
   * @param {string} applicationPath Absolute path to the ScribeTrace binary.
   * @param {object} customArgs Additional command line arguments to pass to the binary.
   */
  async startSession(applicationPath, customArgs = []) {
    const capabilities = {
      capabilities: {
        alwaysMatch: {
          'tauri:options': {
            application: applicationPath,
            args: customArgs
          }
        }
      }
    };

    const value = await this._request('POST', '/session', capabilities);
    this.sessionId = value.sessionId || value.capabilities?.sessionId;
    if (!this.sessionId) {
      throw new Error('WebDriver did not return a valid sessionId upon session creation.');
    }
    return this.sessionId;
  }

  /**
   * Terminates the current WebDriver session.
   */
  async stopSession() {
    if (!this.sessionId) return;
    try {
      await this._request('DELETE', `/session/${this.sessionId}`);
    } catch {
      // Ignore errors on clean teardown
    } finally {
      this.sessionId = null;
    }
  }

  /**
   * Finds an element using standard CSS selector.
   * @param {string} selector CSS selector.
   * @returns {string|null} Element ID or null if not found.
   */
  async findElement(selector) {
    if (!this.sessionId) throw new Error('No active WebDriver session.');
    try {
      const res = await this._request('POST', `/session/${this.sessionId}/element`, {
        using: 'css selector',
        value: selector
      });
      // W3C WebDriver element identifier key
      return res['element-6066-11e4-a52e-4f735466cecf'] || Object.values(res)[0];
    } catch {
      return null;
    }
  }

  /**
   * Finds all elements matching a CSS selector.
   * @param {string} selector CSS selector.
   * @returns {Array<string>} List of Element IDs.
   */
  async findElements(selector) {
    if (!this.sessionId) throw new Error('No active WebDriver session.');
    try {
      const res = await this._request('POST', `/session/${this.sessionId}/elements`, {
        using: 'css selector',
        value: selector
      });
      return res.map(el => el['element-6066-11e4-a52e-4f735466cecf'] || Object.values(el)[0]);
    } catch {
      return [];
    }
  }

  /**
   * Clicks on an element.
   * @param {string} elementId W3C Element ID.
   */
  async click(elementId) {
    if (!this.sessionId) throw new Error('No active WebDriver session.');
    return this._request('POST', `/session/${this.sessionId}/element/${elementId}/click`, {});
  }

  /**
   * Gets visible inner text of an element.
   * @param {string} elementId W3C Element ID.
   * @returns {string} Element text.
   */
  async getText(elementId) {
    if (!this.sessionId) throw new Error('No active WebDriver session.');
    return this._request('GET', `/session/${this.sessionId}/element/${elementId}/text`);
  }

  /**
   * Enters text value into an input element.
   * @param {string} elementId W3C Element ID.
   * @param {string} text Text to type.
   */
  async sendKeys(elementId, text) {
    if (!this.sessionId) throw new Error('No active WebDriver session.');
    return this._request('POST', `/session/${this.sessionId}/element/${elementId}/value`, {
      text: text,
      value: Array.from(text)
    });
  }

  /**
   * Executes synchronous JavaScript inside the webview context.
   * @param {string} script JavaScript code snippet.
   * @param {Array} args Arguments array.
   */
  async executeScript(script, args = []) {
    if (!this.sessionId) throw new Error('No active WebDriver session.');
    return this._request('POST', `/session/${this.sessionId}/execute/sync`, {
      script,
      args
    });
  }

  /**
   * Polls until an element matching the selector appears in the DOM.
   * @param {string} selector CSS selector.
   * @param {number} timeoutMs Maximum wait time in milliseconds.
   * @param {number} intervalMs Poll interval in milliseconds.
   */
  async waitForElement(selector, timeoutMs = 10000, intervalMs = 200) {
    const start = Date.now();
    while (Date.now() - start < timeoutMs) {
      const el = await this.findElement(selector);
      if (el) return el;
      await new Promise(r => setTimeout(r, intervalMs));
    }
    throw new Error(`Timed out waiting ${timeoutMs}ms for element matching: "${selector}"`);
  }

  /**
   * Polls until an element's text includes the expected substring.
   * @param {string} selector CSS selector.
   * @param {string} expectedSubstr Substring to check for.
   * @param {number} timeoutMs Maximum wait time in milliseconds.
   */
  async waitForText(selector, expectedSubstr, timeoutMs = 10000) {
    const start = Date.now();
    let lastText = '';
    while (Date.now() - start < timeoutMs) {
      const el = await this.findElement(selector);
      if (el) {
        lastText = await this.getText(el);
        if (lastText && lastText.includes(expectedSubstr)) {
          return lastText;
        }
      }
      await new Promise(r => setTimeout(r, 200));
    }
    throw new Error(`Timed out waiting ${timeoutMs}ms for text "${expectedSubstr}" in "${selector}". Last text seen: "${lastText}"`);
  }

  /**
   * Takes a base64 screenshot of the current window.
   */
  async takeScreenshot() {
    if (!this.sessionId) throw new Error('No active WebDriver session.');
    return this._request('GET', `/session/${this.sessionId}/screenshot`);
  }
}
