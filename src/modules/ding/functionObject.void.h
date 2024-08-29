#ifndef VOIDED_OBJECT_H
#define VOIDED_OBJECT_H

#include <string>
#include <unordered_map>
#include <functional>

class VoidedObject {
public:
    VoidedObject();
    ~VoidedObject();

    void setProperty(const std::string& key, const std::string& value);
    std::string getProperty(const std::string& key) const;
    bool hasProperty(const std::string& key) const;
    void removeProperty(const std::string& key);

    void applyToFunction(const std::string& functionName, const std::function<void()>& func);

private:
    std::unordered_map<std::string, std::string> properties;
    std::unordered_map<std::string, std::function<void()>> functions;
};

#endif // VOIDED_OBJECT_H
